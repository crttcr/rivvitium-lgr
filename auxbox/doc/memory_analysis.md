
# Rivvitium memory analysis

## High-level plan

- DHAT to monitory heap characteristics
- Consider jemalloc or more efficient memory allocator at some future point

Start with **DHAT**
NOTE: *This assumes that Criterion* is already defined for benchmarking

### Benchmarking with Criterion

1: Add the DHAT dependency
#### **`Cargo.toml`**
```toml
[dev-dependencies]
dhat = "0.4"
```

2: Write a benchmark

#### **`benches/memory.rs`**
```rust
use dhat::{Dhat, DhatAlloc};
#[global_allocator]
static A: DhatAlloc = DhatAlloc;

use your_crate::process;
fn bench_memory(c: &mut Criterion) {
    let _profiler = Dhat::start_heap_profiling();
    let data = /* ... */;                                                     // Profiling data is dumped to 
    c.bench_function("memory profile", |b| b.iter(|| process(&data)));        // `dhat-*.json` for offline analysis
}
```

4. Run with the benchmark
```bash
$ cargo bench
```

#### How to evaluate output

**TBD**

