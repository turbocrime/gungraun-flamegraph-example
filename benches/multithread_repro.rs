//! Multi-threaded flamegraph reproduction.
//!
//! Three variants to observe how `total_flamegraph_map_from_parsed` merges
//! separate per-thread `.out` files into one flamegraph:
//!
//! - **half**: 1 worker calling `top(n)`, bench thread just joins.
//! - **full**: 2 workers both calling `top(n)`, bench thread joins.
//! - **hybrid**: 1 worker + bench thread both calling `top(n)`.

use std::hint::black_box;

use flamegraph_example::top;
use gungraun::{
    library_benchmark, library_benchmark_group, main, Callgrind, EntryPoint, FlamegraphConfig,
    LibraryBenchmarkConfig,
};

#[library_benchmark]
#[bench::case()]
fn bench_half() -> u64 {
    let handle = std::thread::spawn(move || top(black_box(4000)));
    black_box(handle.join().unwrap())
}

#[library_benchmark]
#[bench::case()]
fn bench_full() -> u64 {
    let h1 = std::thread::spawn(move || top(black_box(4000)));
    let h2 = std::thread::spawn(move || top(black_box(4000)));
    black_box(h1.join().unwrap().wrapping_add(h2.join().unwrap()))
}

#[library_benchmark]
#[bench::case()]
fn bench_hybrid() -> u64 {
    let handle = std::thread::spawn(move || top(black_box(4000)));
    let main_result = top(black_box(4000));
    black_box(main_result.wrapping_add(handle.join().unwrap()))
}

library_benchmark_group!(
    name = multithread;
    benchmarks = bench_half, bench_full, bench_hybrid
);

main!(
    config = LibraryBenchmarkConfig::default()
        .tool(
            Callgrind::default()
                .entry_point(EntryPoint::Custom(String::from("flamegraph_example::top")))
                .flamegraph(FlamegraphConfig::default())
        );
    library_benchmark_groups = multithread
);
