# tfdoc

This project aims at generating Terraform module documentation.

`tfdoc` will parse all the `.tf` files within a module's directory and generate markdown code blocks for each resource, data, variable and output block.
It will also include any comments that are placed directly above the block.

> **_NOTE: If no output options are specified, the application will still process the files, but will not produce any output._**

## Installation

You can build the executable yourself with [Rust](https://rust-lang.org).

```sh
# Installing Rust:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Building the application from the repository directory:
cargo build --release

# Alternatively, you can install the application directly:
cargo install --path .
```

The resulting binary can be found in `target/release/` and can be moved to somewhere in your path, for example `/usr/local/bin`.
If you use `cargo install`, the binary will be installed in `~/.cargo/bin` by default.

The Releases section of this repository also contains binary builds for various platforms.

## Usage

```sh
$ tfdoc <DIR(S)> [OPTIONS]

Arguments:
  [DIR(S)]  The directories to process. [default: .]

Options:
  -l, --list [<FILE>]   Output the results as lists in a Markdown file. Default file name: tfdoc_summary_lists.md
  -t, --table [<FILE>]  Output the results as tables in a Markdown file. Default file name: tfdoc_summary_tables.md
  -c, --csv [<FILE>]    Output to a CSV file with the name provided. Default file name: tfdoc_summary.csv
  -q, --quiet           Suppress output and silently proceess inputs
  -r, --recurse         Recursively process directories
  -h, --help            Print help (see more with '--help')
  -V, --version         Print version
```

|Option|Description|
|------|-----------|
|`-c <csv_filenaame>`, `--csv <csv_filename>`|Export the results as a CSV file. Default file name if none is supplied: `tfdoc_summary.csv`|
|`-l <list_filename>`, `--table <list_filename>`|Export the results as a set of markdown lists into the file supplied. Default file name if none is supplied: `tfdoc_summary_lists.md`|
|`-t <table_filename>`, `--table <table_filename>`|Export the results as a set of markdown tables into the file supplied. Default file name if none is supplied: `tfdoc_summary_tables.md`|
|`-q`, `--quiet`|Do not produce any output other than error messages if something goes wrong.|
|`-r`, `--recurse`|Recursively process directories below the one specified.|
|`-h`, `--help`|Prints help in short form with `-h` and long form with `--help`|
|`-V`, `--version`|Prints version information|

You can specify multiple target directories like this:

```sh
tfdoc tests/simple tests/test-fixtures
```

## Expected Terraform File Formatting

The application will only process files with the `.tf` extension. It will ignore all other files.

If you wish to include a header for the file, you can do so by adding a comment at the top of the file. The comment must start with `# Title:` or `// Title:` and must be followed by a space. The comment can be followed by any text you wish to include:

```hcl
# Title: The name of the module
# Top comment prefixed by "Title: " and the following lines
# will be at the top of the Markdown file
```

Any resources, data, variables and output can also be documented using comments. The comment must start with `#` or `//` and must be followed by a space. The comment can be followed by any text you wish to include:

```hcl
# tfdoc keeps comments right on top of resource,
# variable, and output blocks.
resource "aws_instance" "this" {
  # stuff
}
```

If you include a `description` within an element, this will also be processed:

```hcl
// We can have both comments on top
output "name" {
  description = "and within outputs and variables"
}
```

Orphaned (i.e., not associated with a resource, data, variable or output) comments will be ignored.

## Troubleshooting

You can export some logging information by setting the `TFDOC_LOG` environment variable to one of `trace`, `debug`, `info`, `warn`, `error`. If nothing is specified, the application assumes `info`. Currently, only `debug` and `trace` will produce any additional output.

### Logging Examples

Starting the application with the environment variable specified can be done like this:

- `TFDOC_LOG=debug tfdoc .`
- `TFDOC_LOG=trace tfdoc test/simple/ -t -c result.csv`

## Acknowledgements

- This builds on the original [`tfdoc`](https://github.com/maur1th/tfdoc) by [Thomas Maurin](https://github.com/maur1th)
