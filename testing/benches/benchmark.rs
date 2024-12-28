// benches/benchmark.rs
use criterion::{criterion_group, criterion_main, Criterion};

// Function to benchmark
fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}

// Benchmark function
fn benchmark_add_two_numbers(c: &mut Criterion) {
    c.bench_function("add_two_numbers", |b| {
        // Benchmark the function by repeatedly running it
        b.iter(|| add_two_numbers(1, 3))
    });
}

// Group all benchmarks
criterion_group!(benches, benchmark_add_two_numbers);

// Main function to run the benchmarks
criterion_main!(benches);
