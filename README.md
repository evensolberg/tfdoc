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
```

The resulting binary can be found in `target/release/` and can be moved to somewhere in your path, for example `/usr/local/bin`

## Usage

```sh
tfdoc <DIR(S)> [OPTIONS]

Arguments:
  [DIR(S)]...  One or more directories to process. [default: .]

Options:
  -t, --table      Output the results as a table (rather than a list).
  -c, --csv <csv>  Output to a CSV file with the name provided.
  -h, --help       Print help (see more with '--help')
  -V, --version    Print version
```

You can specify more than one directory thusly: `tfdoc dir1/ dir2/ dir3/`. Depending on your operating system, you can also use the `**` glob: `tfdoc modules/**`.

|Option|Description|
|------|-----------|
|`-t`, `--table`|Outputs the results as a set of markdown tables instead of lists|
|`-c <csv_filenaame>`, `--csv <csv_filename>`|Output there results as a CSV file. Example: `--csv my_module.csv`|
|`-h`, `--help`|Prints help in short form with `-h` and long form with `--help`|
|`-V`, `--version`|Prints version information|

### Examples

#### List

```sh
$ tfdoc tests/simmple/

# The name of the module

Top comment prefixed by "Title: " and the following lines
will be at the top of the Markdown file

## Resources

* tests/simple/variables.tf : `aws_instance.this`: tfdoc keeps comments right on top of resource, variable and output blocks. All variables and outputs are kept. Only resources with comments on top are.
* tests/simple/variables.tf : `aws_instance.no_comment_here`:

## Data

* tests/simple/variables.tf : `aws_ami.node`: Data blocks are not ignored

## Inputs

* `environment`: Variable descriptions will be parsed

## Outputs

* `name`: We can have both comments on top and within outputs and variables

## Files

* `tests/simple/variables.tf`
```

#### Tables with CSV

```sh
$ tfdoc tests/simple -t -c result.csv

# The name of the module

Top comment prefixed by "Title: " and the following lines
will be at the top of the Markdown file

## Resources

|Filename|Resource|Description|
|-----|---------|
|tests/simple/variables.tf|`aws_instance.this`|tfdoc keeps comments right on top of resource, variable and output blocks. All variables and outputs are kept. Only resources with comments on top are.|
|tests/simple/variables.tf|`aws_instance.no_comment_here`||

## Data

|Filename|Data|Description|
|-----|---------|
|tests/simple/variables.tf|`aws_ami.node`|Data blocks are not ignored|

## Inputs

|Input|Description|
|-----|---------|
|`environment`|Variable descriptions will be parsed|

## Outputs

|Output|Description|
|-----|---------|
|`name`|We can have both comments on top and within outputs and variables|

## Files

|File Name|Description|
|-----|---------|
|`tests/simple/variables.tf`||

```

This also produces the `result.csv` file as follows:

```csv
Filename,Type,Name,Description
tests/simple/variables.tf,Variable,environment,"Variable descriptions will be parsed"
tests/simple/variables.tf,Resource,aws_instance.this,"tfdoc keeps comments right on top of resource, variable and output blocks. All variables and outputs are kept. Only resources with comments on top are."
tests/simple/variables.tf,Resource,aws_instance.no_comment_here,""
tests/simple/variables.tf,Output,name,"We can have both comments on top and within outputs and variables"
tests/simple/variables.tf,Data,aws_ami.node,"Data blocks are not ignored"
```

This allows for easy import into other tools for query or filtering.

## Troubleshooting

You can export some logging information by setting the `TFDOC_LOG` environment variable to one of `trace`, `debug`, `info`, `warn`, `error`. If nothing is specified, the application assumes `info`.

### Logging Examples

Starting the application with the environment variable specified can be done like this:

- `TFDOC_LOG=debug tfdoc .`
- `TFDOC_LOG=trace tfdoc test/simple/ -t -c result.csv`

## Acknowledgements

- This builds on the original [`tfdoc`](https://github.com/maur1th/tfdoc) by [Thomas Maurin](https://github.com/maur1th)
