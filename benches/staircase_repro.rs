//! Reproduction of the flamegraph staircase bug.
//!
//! Calls `top(4000)` from `src/lib.rs`, which branches into two peaks:
//! peak A (13 frames deep, `a01_c` → … → `a12_c`) and peak B (12 frames
//! deep, `b01_c` → … → `b11_c`). Each level calls three children: `_l`
//! (left work), `_c` (deeper call), `_r` (right work).

use std::hint::black_box;

use flamegraph_example::top;
use gungraun::{
    library_benchmark, library_benchmark_group, main, Callgrind, FlamegraphConfig,
    LibraryBenchmarkConfig,
};

#[library_benchmark]
#[bench::case()]
fn bench_request() -> u64 {
    black_box(top(black_box(4000)))
}

library_benchmark_group!(
    name = staircase_bug;
    benchmarks = bench_request
);

main!(
    config = LibraryBenchmarkConfig::default()
        .tool(
            Callgrind::default()
                .flamegraph(FlamegraphConfig::default())
        );
    library_benchmark_groups = staircase_bug
);
