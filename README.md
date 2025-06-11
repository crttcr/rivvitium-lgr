# Rivvitium

An easy to use, high performance data pipeline with format flexibility written in [Rust](https://www.rust-lang.org/)

## About

Rivvitium is both a library and a tool.  The libray exists to abstract away source and destination format detail and to provide an 
**intermediate data representation** enabling analysis, validation, and statistics.
The tool exists to make using the library functions easy to access.
Fire up the executable and you can be handling data files immediately
without getting bogged down in parsing arcana.

<!---
This is how you can use comments in markdown.

### Installation

This is a link [crates.io](https://crates.io/crates/jsrmx) blah blah

```bash
cargo install jsrmx
```

Then `jsrmx` is executable from your shell

```bash
jsrmx --help
```
-->

## Usage

The Rivvitium application is GUI based so simply launch the executable.

```sh
riv [input]
```
Once running, the choices should be self explanatory:

1. `ingest`   - click the *Ingest file* button or drop a file in the dropzone to select an input file
2. `analyze`  - optionally perform analysis on the file's contents
3. `publish`  - land your data
	* click *Publish*, 
	* choose a destination type, then
	* configure appropriate settings

