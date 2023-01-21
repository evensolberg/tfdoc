//! The main binary file responsible for parsing the Terraform files and outputting the Markdown code.
//!
//! Usage: `tfdoc [-t] PATH`
//!
//! If PATH is omitted, the current directory is used.
//!
//! Use the `-t` parameter to output the documentation in table format rather than list format.

#![forbid(unsafe_code)]

// use std::error::Error;

use std::env;
use std::io;
use std::path::Path;

use env_logger::Env;

// Locally defined crates

mod cli;
mod parser;
mod printer;
mod types;
mod util;

/// The main function responsible for actually carrying out the work.
fn run_app() -> io::Result<()> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = cli::build_cli(env!("CARGO_PKG_VERSION")).get_matches();

    // If the -t parameter has been supplied, output the contents as tables
    let use_tables = cli_args.get_flag("table");
    log::debug!("Using tables: {use_tables}");

    // Look for the path or just use the current directory if none is given
    let path_arg = cli_args
        .get_one::<String>("dirs")
        .map(|s| s.as_str())
        .unwrap_or(".");
    log::debug!("path_arg = {path_arg}");

    // Find just the Terraform files
    let tf_files = util::list_tf_files(Path::new(&path_arg))?;

    // Parse the files found and put them into a list
    let mut result: Vec<types::DocItem> = vec![];

    for tf_file in &tf_files {
        result.append(&mut parser::parse_hcl(tf_file.clone())?);
        log::debug!("main::result = {:?}", result);
    }

    // Output the resulting markdown
    printer::render(&result, use_tables);
    printer::print_files(&tf_files, use_tables);

    // Return safely
    Ok(())
}

/// Calls `run_app` and exits with error code `0` if successful. Otherwise prints an error message to `stderr` and exits with error code `1`.
fn main() {
    // Enable logging
    let env = Env::default().filter_or("TFDOC_LOG", "info");

    env_logger::init_from_env(env);

    // Run the application
    ::std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {err}");
            1
        }
    });
}
