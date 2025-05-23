## Project Name

**Rivvitium – Rust Data Pipeline**

## Project Description

Ever wondered how you’ve managed to live without a modular data pipeline to ingest and emit data across a variety of formats? In this project, we’ll build a high-performance data parsing and extraction library in Rust with interchangeable front-end reader modules (such as CSV, JSON, XML, Excel) and back-end writer modules (CSV, SQLite, HTTP Posts, etc.). You’ll be able to mix and match readers and writers as needed—dropping in new formats without touching your core logic—and then showcase this flexibility in a desktop application that dynamically configures connectors and transforms files on the fly.

Rust is the perfect choice here: its zero-cost abstractions and strong type system let you write parsers that run as fast as C but never segfault. You’ll start by defining a common, trait-based API for data sources and sinks, then implement each format as a plug-in module. Along the way you’ll learn how to:

* Design a clean, extensible trait hierarchy for data sources and sinks and the all-important intermediate data representation that connect them

* Stream huge files without exploding memory, using Rust’s iterator and async I/O patterns

* Handle edge cases like malformed rows, nested JSON & XML structures, zip file bundles, merged Excel cells, and more

* Bundle everything into a reusable crate with comprehensive unit and integration tests

What brings the power is a custom Intermediate Representation (IR) for data blocks. We’ll use tried-and-true compiler design to create versatile, reusable data extractors and a variety of data publishers that work with our IR. If the library itself isn’t exciting enough, a modern front-end application will bring the heat. We’ll end up with a composable library we can embed in any Rust project, plus a desktop app that makes exploring data pipelines a breeze. Whether you’re migrating datasets, prototyping workflows, or building custom reporting tools, this project gives you both under-the-hood flexibility and a user-friendly GUI to prove it.

Our application will let users select input files, pick an output target, and watch progress as rows flow through the pipeline and out to the destination. This demo will illustrate real-world usage—data analysts, reporting tools, and migration scripts can all be powered by your work.

## **Project Objectives:**

* **Design a pluggable architecture**  
* Define a front-end reader API that unifies various data formats  
* Design an Intermediate data representation that enables pluggability  
* Define a back-end writer API that unifies destinations   
* **Implement core format connectors**  
* Build reader plugins for CSV, JSON, XML, and Excel (via calamine)  
* Build writer plugins for CSV and SQLite, and HTTP Posts  
* **Ensure high-performance streaming**  
  * Leverage Rust’s iterators and async I/O to process arbitrarily large files with bounded memory footprint  
  * Measure throughput and performance to  and maximal throughput.  
* **Handle real-world edge cases**  
  *  Add robust support for malformed rows, nested JSON, XML namespaces, merged Excel cells, and error reporting so pipelines don’t break on dirty data.  
* **Package as a reusable Rust crate**  
  *  Organize your code into a well-documented, tested library with clear examples, so other projects can pull in readers and writers as needed.  
* **Build a dynamic desktop demo**  
* Create a GUI application (e.g. with Slint) that discovers and loads reader/writer plugins at runtime, lets users select sources and targets, and visualizes the data flow through the pipeline.

## **Project Requirements:**

**Architecture**

* Define core trait API: Specify \`DataReader\` and \`DataWriter\` traits to abstract over input and output formats.  
* Implement plugin registry: Create a mechanism to register and discover reader and writer modules at runtime.

**Front-end reader modules**

* Implement CSV reader: Parse delimited text with configurable delimiter, quote, and header handling.  
* Implement JSON reader: Stream JSON arrays and objects into a uniform record format.

**Back-end writer modules**

* Implement CSV writer: Serialize records with configurable delimiter and quoting rules.  
* Implement SQLite writer: Use \`rusqlite\` to create tables and insert rows, mapping types appropriately.

**Streaming and performance**

* Enable streaming reads: Process input iterators so that large files never fully load into memory.  
* Support asynchronous I/O: Allow readers and writers to run in async contexts for nonblocking pipelines.  
* Benchmark throughput: Provide example benchmarks to verify that performance meets targets.

**Error handling and data validation**

* Define error types: Create a unified error enum for parse, I/O, and schema-mismatch failures.  
* Implement recoverable errors: Allow pipelines to skip bad records with configurable logging.

**Packaging and testing**

* Organize as a Cargo workspace: Separate readers and writers crates  
* Write unit tests: Cover trait implementations, edge cases, and error conditions.  
* Write integration tests: Verify end-to-end flows from readers to writers with sample files.  
* Generate documentation: Include examples showing how to plug in custom modules.

**Demonstration desktop application**

* Choose a GUI toolkit: Use Slint (or another Rust GUI) to build the front end.  
* Develop a configuration mechanism to manage application configuration  
* Implement dynamic plugin loading: Detect available reader/writer crates and present them in the UI.  
* Build drag-and-drop workflow: Let users drop files, select an output target, and visualize progress.  
* Display results: Show a summary of processed records and any errors encountered.

## **Project Milestones**:

### Week 1

1. Understand fundamentals  
* Choose crates for inbound and outbound data processing  
* Refine requirements for the intermediate data representation (IDR)  
2. Performance Measurement  
* Research performance measurement in rust applications and libraries  
* Decide which metrics are important to capture  
* Research tracing crate for capturing metrics  
* Learn profiling tools  
3. Project Layout  
* Figure out what support modules are going to be needed, e.g. configuration  
* Design a starting project module structure  
4. Notify your instructor  
* Inform your instructor on Discord that you have chosen this project

### Week 2

1. Infrastructure:  
* Decide how to pass data through the pipeline  
* Figure out how to measure performance  
2. Components:  
* Design and implement IR as a sum type  
* Add error messages as enum constants  
* Create CSV data extractor  
* Create JSON data extractor  
* Create Statistics collecting observer  
* Create CSV consumer  
* Create Sqlite consumer

### Week 3

1. Refinement:  
* Create a suite of integration tests  
* Measure baseline performance  
* Profile performance and identify areas for improvement  
2. Application Design:  
* Design UI screens for Rivvitium  
* Research how to perform drag and drop  
* Choose icons for design  
* Incorporate UI toolkit into workbench

### Week 4

1. Create Rivvitium Application:  
   * Create a settings page for configuration  
   * Create mechanism for drag and drop  
   * Create a page to specify data pipeline  
   * Create a feedback page/window to display progress  
   * Create a completion UI component

### Week 5

1. Refinement:  
   1. Handle error conditions in the application  
   2. Test with large files to ensure bounded memory  
2. Parallelism:  
   1. Convert input data into chunks  
   2. Choose a mechanism for work distribution  
   3. Implement parallel processing  
   4. Measure performance deltas

## Optional Enhancements:

* **Extend Reader Implementations:**  
  * Implement XML reader: Support element- and attribute-based mapping, including namespaces.  
  * Implement Excel reader: Use \`calamine\` to load \`.xls\` and \`.xlsx\`, handling merged cells and multiple sheets.  
  * Implement ZipFile reader that contains multiple files  
* **Extend Writer Implementations**  
  * Implement HTTP Post writer: Use \`reqwest\` to create post data to an HTTP endpoint.  
* **IR Transformations**  
  * Add schema validation hooks: Let users supply simple rules (e.g., required fields, type checks).
