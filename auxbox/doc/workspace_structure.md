## Workspace structure

### Objective

Create a workspace that is comprised of multiple crates to keep the codebase organized
The initial work targets creating a library of data processing resources that permit
reading and writing structured data files. Given Rivvitium's multi-format nature, reading
and writing should be independent crates so that additions can be isolated in crate versions.

### Straw man directory structure 
This will definitely change but serves as a starting point for discussion and consderation.


```
.
├── Cargo.toml          # Workspace root (virtual manifest)
├── .env                # Environment variables
├── auxbox/             # Auxillary resources: scripts, data, doc
│   └── csv_to_csv.sh
│   └── csv_to_sqlite.sh
│
│
├── crates/             # All individual Rust crates
│   ├── common/         # Library: Shared data structures, utilities, error types
│   │   └── src/
│   │       └── lib.rs
│   │
│   ├── pipeline/       # Library: Core processing logic, transformations, orchestration
│   │   └── src/
│   │       └── lib.rs
│   │
│   ├── data_reader/    # Library: Reads data from source, feeds into pipeline
│   │   └── src/
│   │       └── lib.rs
│   │
│   └── data_writer/    # Library: Writes processed data to destination
│       └── src/
│           └── lib.rs
│
├── benches/            # Performance benchmarks
│   └── benchmarks.rs
│
│
├── config/             # Centralized configuration files
│   └── default.yaml
│
│
└── tests/              # End-to-end integration tests
    └── integration_tests.rs
```
