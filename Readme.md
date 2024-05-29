# Composable

[![Crates.io](https://img.shields.io/crates/v/crate-name.svg)](https://crates.io/crates/composable)
[![Documentation](https://docs.rs/composable/badge.svg)](https://docs.rs/composable)
[![License](https://img.shields.io/crates/l/composable.svg)](https://github.com/selfsupervised-ai/composable/blob/main/LICENSE)

A brief description of what your crate does.

## Features

This project aims to develop composable abstractions for high-performance code generation within MLIR. The proposed abstractions and transformations offer both immediate and long-term benefits. Our approach involves breaking down generic computations into smaller tiles, utilizing their algebraic properties and structure. These computations can be fused and gradually reduced into loops over vector primitives, which can be retargeted. They apply to both immutable tensor values and in-memory buffers that may have side effects. These abstractions support storage formats, including dense, sparse, and quantized representations. An in-place bufferization pass ensures efficient memory usage by materializing programs in tensor form, even those transformed through tiling, fusion, and other processes. These practical benefits underscore the value and relevance of our proposed abstractions and transformations.

Our approach preserves high-level, domain-specific information, preventing the premature loss of the computational structure. This enables transformations without the performance limitations of numerical libraries when a fused operation lacks a high-performance implementation. These transformations can lower operations to hardware instructions that implement coarse-grained vector operations or to numerical libraries like Eigen, serving as a fallback. This flexible and adaptable approach enhances compiler transformations and opens new possibilities for compiler-library co-design.

Additionally, tiled operations focus on subsets of tensor values or memory buffers, leveraging the natural structure in tensor operations while remaining generic in tensor representation (values or side effects, vectors or scalars, dense or sparse) and decomposition methods (various tiling forms). This improves composability by allowing transformations to apply to individual or group operations rather than entire loops or control-flow graphs. It also simplifies complex transformations' expression and lowers sequences, facilitating autotuning.

The intermediate representation remains executable at any intermediate transformation and lowering step, greatly simplifying debugging, testing, and performance evaluation, and blurring the lines between the programmer's and the compiler's responsibilities.

## Installation

Add the following line to your `Cargo.toml` file:
composable = "0.1.0"

## Notes for CI

### Testing

- Write Tests for all functions, including edge cases
- Miri: detects undefiuned behaviour and leaks, even if nothing panics
- Sanitizers: detects problematic threading and memory access patterns
- Embrace Chaos: turmoil/shuttle (async/sync chaos), quickcheck/proptest (value chaos), cargo-mutants (logic chaos)
- Be exhaustive when possible: Loom (all possible and distinguishable concurrent executions), Kani (all possible and distinguishible inputs). Only possible for core primitives.

### Benchmarking and Profiling

- Use profilers to detect problem areas and prioritize experimentation. flamegraph, samply, counts, coz (causal profiling)
- Benchmark tool: Criterion, Devin, Hyperfine, Bencher
- Capture these in the benchmark: Pathological cases, micro and macro, under at, and over capacity, on all relevant targets, usefulness, throughput, memory usage, latency.
- Use statistical tests (some implmented in Criterion) for comparisons, and not standard pointwise comparisions.
- iai-callgrind, tango

### Documentation

- Decisions taken: Which alternatives were discarded and why? Which tradeoffs were accepted and why?
- (Y)ADRs: tools for documentation
- Missing handling of corner-cases. todo!()/unimplemented!(), Future optimization opportunities: Absence of an impl (like From)

### Make misuse inexpressible

- Newtypes (not aliases): Meters(u64) vs Miles(u64)
<!-- trunk-ignore(markdownlint/MD033) -->
- Typestates: Rocket <Ground> vs Rocket <Air> Maybe can be used for train vs inference
- Two-phase Structs: TomConfig vs ResolvedConfig
- Enums over Booleans
- Enums for linked arguments: f(true, Some(\_)) (+) f(false, None) should be f(enums, Option)

### Follow idoms

- Clippy is your friend
- The Rust API Guidelines
- Try to use Rust features instead of trying to re-interpret Python or C/C++

### Minimize Hazards

- Concrete types: prefer -> Impl Trait, avoid pub fields
- args, return types, train impls, ...
- impl From: prefer non-pub inherent methods!
- cargo-semver-checks, cargo-public-api, cargo-vet are tools for automatic change detection, but are not perfect.

### Make stagnation a recurrent choice

- Have reminders when there are changes in dependencies.
- Auto-merging bump PRs
- Dependabot
- Upstream changes (no forks!)
- Wrap unstable dependencies.

cmake -G Ninja ../llvm \
 -DLLVM_ENABLE_PROJECTS=mlir \
 -DLLVM_BUILD_EXAMPLES=ON \
 -DLLVM_TARGETS_TO_BUILD="Native;ARM;Mips" \
 -DCMAKE_BUILD_TYPE=Release \
 -DLLVM_ENABLE_ASSERTIONS=ON \
 -DCMAKE_C_COMPILER=clang -DCMAKE_CXX_COMPILER=clang++ \
 -DLLVM_CCACHE_BUILD=OFF \
 -DLLVM_USE_SANITIZER="Address;Undefined" \
 -DMLIR_INCLUDE_INTEGRATION_TESTS=ON
