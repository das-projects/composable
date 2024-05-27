use melior::{
    dialect::{arith, func, DialectRegistry},
    ir::{
        attribute::{StringAttribute, TypeAttribute},
        r#type::FunctionType,
        *,
    },
    pass,
    utility::{register_all_dialects, register_all_llvm_translations},
    Context, ExecutionEngine,
};
use std::time::Instant;

fn load_all_dialects(context: &Context) {
    let registry = DialectRegistry::new();
    register_all_dialects(&registry);
    context.append_dialect_registry(&registry);
    context.load_all_available_dialects();
}

fn create_test_context() -> Context {
    let context = Context::new();

    context.attach_diagnostic_handler(|diagnostic| {
        eprintln!("{}", diagnostic);
        true
    });

    load_all_dialects(&context);
    register_all_llvm_translations(&context);

    context
}

fn invoke_packed() {
    let context = create_test_context();

    let mut module = Module::parse(
        &context,
        r#"
        module {
            func.func @add(%arg0 : i32) -> i32 attributes { llvm.emit_c_interface } {
                %res = arith.addi %arg0, %arg0 : i32
                return %res : i32
            }
        }
        "#,
    )
    .unwrap();

    let pass_manager = pass::PassManager::new(&context);
    pass_manager.add_pass(pass::conversion::create_func_to_llvm());

    pass_manager
        .nested_under("func.func")
        .add_pass(pass::conversion::create_arith_to_llvm());

    assert_eq!(pass_manager.run(&mut module), Ok(()));

    let engine = ExecutionEngine::new(&module, 2, &[], false);

    let mut argument = 42;
    let mut result = -1;

    let now = Instant::now();
    assert_eq!(
        unsafe {
            engine.invoke_packed(
                "add",
                &mut [
                    &mut argument as *mut i32 as *mut (),
                    &mut result as *mut i32 as *mut (),
                ],
            )
        },
        Ok(())
    );
    let elapsed = now.elapsed();

    assert_eq!(result, 84);
    assert_eq!(argument, 42);
    println!("Generated MLIR:\n{}", module.as_operation());
    println!("Result: {} calculated in time {:.2?}", result, elapsed);
}

fn main() {
    // We need a registry to hold all the dialects
    let registry = DialectRegistry::new();
    // Register all dialects that come with MLIR.
    register_all_dialects(&registry);

    // The MLIR context, like the LLVM one.
    let context = Context::new();
    context.append_dialect_registry(&registry);
    context.load_all_available_dialects();

    // A location is a debug location like in LLVM, in MLIR all
    // operations need a location, even if its "unknown".
    let location = Location::unknown(&context);

    // A MLIR module is akin to a LLVM module.
    let module = Module::new(location);

    // A integer-like type with platform dependent bit width. (like size_t or usize)
    // This is a type defined in the Builtin dialect.
    let index_type = Type::index(&context);

    // Append a `func::func` operation to the body (a block) of the module.
    // This operation accepts a string attribute, which is the name.
    // A type attribute, which contains a function type in this case.
    // Then it accepts a single region, which is where the body
    // of the function will be, this region can have
    // multiple blocks, which is how you may implement
    // control flow within the function.
    // These blocks each can have more operations.
    module.body().append_operation(func::func(
        // The context
        &context,
        // accepts a StringAttribute which is the function name.
        StringAttribute::new(&context, "add"),
        // A type attribute, defining the function signature.
        TypeAttribute::new(
            FunctionType::new(&context, &[index_type, index_type], &[index_type]).into(),
        ),
        {
            // The first block within the region, blocks accept arguments
            // In regions with control flow, MLIR leverages
            // this structure to implicitly represent
            // the passage of control-flow dependent values without the complex nuances
            // of PHI nodes in traditional SSA representations.
            let block = Block::new(&[(index_type, location), (index_type, location)]);

            // Use the arith dialect to add the 2 arguments.
            let sum = block.append_operation(arith::addi(
                block.argument(0).unwrap().into(),
                block.argument(1).unwrap().into(),
                location,
            ));

            // Return the result using the "func" dialect return operation.
            block.append_operation(func::r#return(&[sum.result(0).unwrap().into()], location));

            // The Func operation requires a region,
            // we add the block we created to the region and return it,
            // which is passed as an argument to the `func::func` function.
            let region = Region::new();
            region.append_block(block);
            region
        },
        &[],
        location,
    ));

    // Verify the module, this will check if the module is well-formed.
    assert!(module.as_operation().verify());
    println!("Generated MLIR:\n{}", module.as_operation());
    invoke_packed();
}
