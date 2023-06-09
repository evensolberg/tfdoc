# tfdoc

This project aims at generating Terraform module documentation.

tfdoc will parse all the `.tf` files within a module's directory and generate markdown code to `stdout`. This can easily be piped to a file: `tfdoc > result.md`

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
$ tfdoc <DIR> [OPTIONS]

Arguments:
  [DIR]  The directory to process. [default: .]

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
|`-c <csv_filenaame>`, `--csv <csv_filename>`|Export the results as a CSV file. Example: `--csv my_module.csv`|
|`-l <list_filename>`, `--table <list_filename>`|Export the results as a set of markdown lists into the file supplied. Default file name if none is supplied is `tfdoc_summary_lists.md`|
|`-t <table_filename>`, `--table <table_filename>`|Export the results as a set of markdown tables into the file supplied. Default file name if none is supplied is `tfdoc_summary_tables.md`|
|`-q`, `--quiet`|Do not produce any output other than error messages if something goes wrong.|
|`-r`, `--recurse`|Recursively process directories below the one specified.|
|`-h`, `--help`|Prints help in short form with `-h` and long form with `--help`|
|`-V`, `--version`|Prints version information|

## Troubleshooting

You can export some logging information by setting the `TFDOC_LOG` environment variable to one of `trace`, `debug`, `info`, `warn`, `error`. If nothing is specified, the application assumes `info`.

### Logging Examples

Starting the application with the environment variable specified can be done like this:

- `TFDOC_LOG=debug tfdoc .`
- `TFDOC_LOG=trace tfdoc test/simple/ -t -c result.csv`

## Acknowledgements

- This builds on the original [`tfdoc`](https://github.com/maur1th/tfdoc) by [Thomas Maurin](https://github.com/maur1th)
