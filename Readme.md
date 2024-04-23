# Composable

[![Crates.io](https://img.shields.io/crates/v/crate-name.svg)](https://crates.io/crates/composable)
[![Documentation](https://docs.rs/composable/badge.svg)](https://docs.rs/composable)
[![License](https://img.shields.io/crates/l/composable.svg)](https://github.com/selfsupervised-ai/composable/blob/main/LICENSE)

A brief description of what your crate does.

## Features

- Feature 1: Description of feature 1.
- Feature 2: Description of feature 2.
- ...

## Installation

Add the following line to your `Cargo.toml` file:
crate-name = "0.1.0"


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
- Typestates: Rocket <Ground> vs Rocket <Air> Maybe can be used for train vs inference
- Two-phase Structs: TomConfig vs ResolvedConfig
- Enums over Booleans
- Enums for linked arguments: f(true, Some(_)) (+) f(false, None) should be f(enums, Option)
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





