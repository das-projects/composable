use std::borrow::Borrow;

use melior::{
    dialect::{arith, func, DialectRegistry},
    ir::{
        attribute::{StringAttribute, TypeAttribute},
        r#type::FunctionType,
        Block, Location, Module, Operation, OperationRef, Region, Type, Value,
    },
    utility::register_all_dialects,
    Context,
};

struct MLIRModule<'a> {
    // The MLIR context, like the LLVM one.
    context: &'a Context,
    // We need a registry to hold all the dialects
    registry: DialectRegistry,
    // A location is a debug location like in LLVM, in MLIR all
    // operations need a location, even if its "unknown".
    location: Location<'a>,
    // A MLIR module is akin to a LLVM module.
    module: Module<'a>,
}

impl<'a> MLIRModule<'a> {
    fn new(context: &'a Context) -> MLIRModule<'a> {
        let registry: DialectRegistry = DialectRegistry::new();
        let location: Location<'a> = Location::unknown(context);
        let module: Module<'a> = Module::new(location.clone());

        register_all_dialects(&registry);
        context.append_dialect_registry(&registry);
        context.load_all_available_dialects();

        MLIRModule {
            context,
            registry,
            location,
            module,
        }
    }

    fn add_binary_op(
        &mut self,
        name: &str,
        binary_ops: impl Fn(Value<'a, '_>, Value<'a, '_>, Location<'a>) -> Operation<'a>,
        args: &'a [MLIRType<'a>],
        ret: &'a [MLIRType<'a>],
    ) {
        // Append a `func::func` operation to the body (a block) of the module.
        // This operation accepts a string attribute, which is the name.
        // A type attribute, which contains a function type in this case.
        // Then it accepts a single region, which is where the body
        // of the function will be, this region can have
        // multiple blocks, which is how you may implement
        // control flow within the function.
        // These blocks each can have more operations.
        self.module.body().append_operation(func::func(
            // The context
            self.context,
            // accepts a StringAttribute which is the function name.
            StringAttribute::new(self.context, name),
            // A type attribute, defining the function signature.
            TypeAttribute::new(FunctionType::new(self.context, args, ret).into()),
            {
                // The first block within the region, blocks accept arguments
                // In regions with control flow, MLIR leverages
                // this structure to implicitly represent
                // the passage of control-flow dependent values without the complex nuances
                // of PHI nodes in traditional SSA representations.
                let block_args: Vec<(Type, Location<'a>)> = args
                    .iter()
                    .map(|ty: &MLIRType<'a>| (ty.index_type, self.location))
                    .collect::<Vec<_>>();
                let block: Block = Block::new(&block_args);

                let ops_call: OperationRef = block.append_operation(binary_ops(
                    block.argument(0).unwrap().into(),
                    block.argument(1).unwrap().into(),
                    self.location,
                ));

                // Return the result using the "func" dialect return operation.
                block.append_operation(func::r#return(
                    &[ops_call.result(0).unwrap().into()],
                    self.location,
                ));
                // The Func operation requires a region,
                // we add the block we created to the region and return it,
                // which is passed as an argument to the `func::func` function.
                let region = Region::new();
                region.append_block(block);
                region
            },
            &[],
            self.location,
        ));
    }

    fn verify(&self) -> bool {
        println!("Generated MLIR:\n{}", self.module.as_operation());
        self.module.as_operation().verify()
    }
    /*
       fn run(&mut self, arg1: Value<'a, '_>, arg2: Value<'a, '_>, ret: Type) {
           let pass_manager = pass::PassManager::new(&self.context);
           pass_manager.add_pass(pass::conversion::create_func_to_llvm());

           pass_manager
               .nested_under("func.func")
               .add_pass(pass::conversion::create_arith_to_llvm());

           assert_eq!(pass_manager.run(&mut self.module), Ok(()));

           self.context.attach_diagnostic_handler(|diagnostic| {
               eprintln!("{}", diagnostic);
               true
           });

           let engine = melior::ExecutionEngine::new(&self.module, 2, &[], false);

           unsafe {
               engine.invoke_packed(
                   "add",
                   &mut [
                       arg1,
                       arg2,
                   ],
               );
           }
       }
    */
}

struct MLIRType<'a> {
    index_type: Type<'a>,
}

impl <'a>MLIRType<'a> {
    fn new(context: &Context) -> MLIRType {
        let index_type: Type = Type::index(context);
        MLIRType { index_type }
    }
}

fn main() {
    let context: Context = Context::new();
    let mut module: MLIRModule = MLIRModule::new(&context);
    let mlirtype: MLIRType = MLIRType::new(&context);

    module.add_binary_op("add", arith::addi, &[mlirtype], &[mlirtype]);

    assert!(module.verify());
}
