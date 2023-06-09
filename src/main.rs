//! The main binary file responsible for parsing the Terraform files and outputting the Markdown code.
//!
//! Usage: `tfdoc -h` for help
#![forbid(unsafe_code)]

use std::env;

use clap::parser::ValueSource;
use env_logger::Env;

// Locally defined crates

mod cli_builder;
mod exporter;
mod parser;
mod printer;
mod types;
mod util;

/// The main function responsible for actually carrying out the work.
fn run_app() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let cli_args = cli_builder::build_cli(env!("CARGO_PKG_VERSION")).get_matches();
    let quiet = cli_args.value_source("quiet") == Some(ValueSource::CommandLine);

    let dirs_to_process: Vec<&str> = cli_args
        .get_many::<String>("dirs")
        .unwrap_or_default()
        .map(std::string::String::as_ref)
        .collect();

    let mut paths_to_process: Vec<String> = vec![];
    if cli_args.value_source("recurse") == Some(ValueSource::CommandLine) {
        for dir in dirs_to_process {
            let dir = std::path::Path::new(dir).to_string_lossy().to_string();
            util::build_directory_list(&dir, &mut paths_to_process);
        }
    } else {
        for dir in dirs_to_process {
            paths_to_process.push(dir.to_string());
        }
    }
    log::debug!("paths_to_process = {paths_to_process:?}");

    let mut tfdoc_result: Vec<types::DocItem> = vec![];
    let mut tf_files_processed = vec![];
    for current_path in paths_to_process {
        let path = std::path::Path::new(&current_path);
        let tf_files = util::list_tf_files(path)?;

        for tf_file in &tf_files {
            tf_files_processed.push(tf_file.clone());
            tfdoc_result.append(&mut parser::parse_hcl(&tf_file.clone())?);
            let tff = tf_file.to_str().unwrap_or("Unknown");
            if !quiet {
                println!("{tff}");
            }
        }
    }

    if let Some(md_list_filename) = cli_args.get_one::<String>("list") {
        exporter::export_markdown(md_list_filename, &tf_files_processed, &tfdoc_result, false)?;
    }

    if let Some(md_table_filename) = cli_args.get_one::<String>("table") {
        exporter::export_markdown(md_table_filename, &tf_files_processed, &tfdoc_result, true)?;
    }

    if let Some(csv_filename) = cli_args.get_one::<String>("csv") {
        let _exp = exporter::export_csv(csv_filename, &tfdoc_result);
    }

    if !quiet {
        printer::print_summary(&tf_files_processed, &tfdoc_result);
    }

    // Return safely
    Ok(())
}

/// Calls `run_app` and exits with error code `0` if successful. Otherwise prints an error message to `stderr` and exits with error code `1`.
fn main() {
    let env = Env::default().filter_or("TFDOC_LOG", "info");
    env_logger::init_from_env(env);

    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {err}");
            1
        }
    });
}
