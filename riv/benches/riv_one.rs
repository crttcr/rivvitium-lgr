
use criterion::{criterion_group, criterion_main, Criterion};

// Example function to benchmark
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

/*/*/*/*/*/*/*/*
fn run_pipeline_test() {
    test_capture_of_start_and_end();
}
*/

// A benchmark function
// You can group benchmarks for better organization in reports
// "fib 30" is another benchmark in the same group
//
fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(20)));
    c.bench_function("fib 30", |b| b.iter(|| fibonacci(30)));
}

// You can create multiple benchmark groups if needed
//
fn bench_another_function(c: &mut Criterion) {
    c.bench_function("some_other_op", |b| b.iter(|| {
        // Code to benchmark
        let _ = "hello".to_string() + " world";
    }));
}


// Register your benchmark functions with criterion
// For larger projects, you might split into multiple groups:
//
//   criterion_group!(name_of_group_1, bench_func_1, bench_func_2);
//   criterion_group!(name_of_group_2, bench_func_3);
//   criterion_main!(name_of_group_1, name_of_group_2);
//
criterion_group!(benches, bench_fibonacci, bench_another_function);

// This macro generates the main function for your benchmarks
//
criterion_main!(benches);
