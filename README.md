# Rivvitium

An easy to use, high performance data pipeline with format flexibility written in [Rust](https://www.rust-lang.org/)

## About

Rivvitium is both a library and a tool.  The libray exists to abstract away source and destination formatting details
and to provide an **intermediate data representation** that enables analysis, validation, and statistics.
The tool exists to make using the library functions easy to access. Fire up the executable and you can be
handling data files immediately.

<!---
your comment goes here
and here

### Installation

This is published to [crates.io](https://crates.io/crates/jsrmx) so you can simply do a global install with:

```sh
cargo install jsrmx
```

Then `jsrmx` is executable from your shell

```sh
jsrmx --help
```

-->

## Usage

The Rivvitium application is GUI based so simply launch the executable.

```sh
rivvitium [input]
```
Once running, the choices should be self explanatory:

1. `ingest`   - click on the drop file button to select an input data file
2. `analyze`  - perform analysis on the file's contents
3. `publish`  - choose the type of destination and configure appropriate settings

