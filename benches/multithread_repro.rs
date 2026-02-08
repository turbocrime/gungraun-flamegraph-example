//! Multi-threaded flamegraph reproduction.
//!
//! Four variants to observe how `total_flamegraph_map_from_parsed` merges
//! separate per-thread `.out` files into one flamegraph. The name encodes
//! how many `top()` calls come from the bench thread vs worker threads:
//!
//! - **zero_thread_one**: bench thread joins only, 1 worker calls `top`.
//! - **one_thread_zero**: bench thread calls `top`, 1 worker does non-`top` work.
//! - **zero_thread_two**: bench thread joins only, 2 workers call `top`.
//! - **one_thread_one**: bench thread calls `top`, 1 worker calls `top`.

use std::hint::black_box;

use flamegraph_example::{top, work};
use gungraun::{
    Callgrind, EntryPoint, FlamegraphConfig, LibraryBenchmarkConfig, library_benchmark,
    library_benchmark_group, main,
};

#[library_benchmark]
#[bench::case()]
fn bench_zero_thread_one() -> u64 {
    let handle = std::thread::spawn(move || top(black_box(4000)));
    black_box(handle.join().unwrap())
}

#[library_benchmark]
#[bench::case()]
fn bench_one_thread_zero() -> u64 {
    let handle = std::thread::spawn(move || work(black_box(4000)));
    let main_result = top(black_box(4000));
    black_box(main_result.wrapping_add(handle.join().unwrap()))
}

#[library_benchmark]
#[bench::case()]
fn bench_zero_thread_two() -> u64 {
    let h1 = std::thread::spawn(move || top(black_box(4000)));
    let h2 = std::thread::spawn(move || top(black_box(4000)));
    black_box(h1.join().unwrap().wrapping_add(h2.join().unwrap()))
}

#[library_benchmark]
#[bench::case()]
fn bench_one_thread_one() -> u64 {
    let handle = std::thread::spawn(move || top(black_box(4000)));
    let main_result = top(black_box(4000));
    black_box(main_result.wrapping_add(handle.join().unwrap()))
}

library_benchmark_group!(
    name = multithread;
    benchmarks =
        bench_zero_thread_one,
        bench_one_thread_zero,
        bench_zero_thread_two,
        bench_one_thread_one
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
