
# Rivvitium benchmarking

## High-level plan

- Benchmarking with Criterion for throughput/latency.
- DHAT or jemalloc_ctl for memory footprints.
- CI guardrails to detect performance drifts.
- Flamegraphs for deeper profiling.

Start with **Benchmarking** and **Memmory Usage** 

### Benchmarking with Criterion

1: Add the Criterion dependency
#### **`Cargo.toml`**
// Cargo.toml
[dev-dependencies]
criterion = "0.4"
```

2: Create "benches" directory at crate root
```bash
cd    $CRATE_ROOT
mkdir benches
```

3: Write a benchmark

#### **`bench_throughput.rs`**
```rust
use criterion::{criterion_group, criterion_main, Criterion};
use your_crate::process;        // your library entrypoint
use your_crate::test_utils;     // helper to generate data

fn bench_throughput(c: &mut Criterion) {
    // prepare a large test file in memory
    let data = test_utils::generate_csv_rows(10_000);
    // measure how many rows/sec your `process` can handle
    c.bench_function("process 10k rows", |b| {
        b.iter(|| proc:ess(&data))
    });
}

criterion_group!(benches, bench_throughput);
criterion_main!(benches);
```

4. Run with the benchmark
```bash
$ cargo bench
```

### Monitoring memory

**TBD**

