//! The main binary file responsible for parsing the Terraform files and outputting the Markdown code.
//!
//! Usage: `tfdoc -h` for help
#![forbid(unsafe_code)]

use std::env;

use clap::parser::ValueSource;
use env_logger::Env;
use walkdir::WalkDir;

use crate::printer::print_summary;

// Locally defined crates

mod cli_builder;
mod exporter;
mod parser;
mod printer;
mod types;
mod util;

/// The main function responsible for actually carrying out the work.
fn run_app() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let cli_args = cli_builder::build_cli(env!("CARGO_PKG_VERSION")).get_matches();
    log::debug!("cli_args = {:?}", cli_args);

    // Determine whether to process quietly or not
    let quiet = cli_args.value_source("quiet") == Some(ValueSource::CommandLine);
    let recurse = cli_args.value_source("recurse") == Some(ValueSource::CommandLine);

    // Determine the directories to process
    let dirs: Vec<&str> = cli_args
        .get_many::<String>("dirs")
        .unwrap_or_default()
        .map(std::string::String::as_ref)
        .collect();

    let mut process_paths: Vec<String> = vec![];

    if recurse {
        for dir in dirs {
            let dir = std::path::Path::new(dir).to_string_lossy().to_string();
            build_directory_list(&dir, &mut process_paths);
        }
    } else {
        for dir in dirs {
            process_paths.push(dir.to_string());
        }
    }
    log::debug!("process_paths = {process_paths:?}");

    // Parse the files found and put them into a list
    let mut result: Vec<types::DocItem> = vec![];
    let mut all_tf_files = vec![];

    // Iterate through the paths supplied
    for current_path in process_paths {
        // Find just the Terraform files
        let path = std::path::Path::new(&current_path);
        let tf_files = util::list_tf_files(path)?;

        // Process the terraform files
        for tf_file in &tf_files {
            all_tf_files.push(tf_file.clone());
            result.append(&mut parser::parse_hcl(&tf_file.clone())?);
            let tff = tf_file.to_str().unwrap_or("Unknown");
            if !quiet {
                println!("{tff}");
            }
        }
    }

    // Output the resulting markdown to file as a list
    if let Some(md_list_filename) = cli_args.get_one::<String>("list") {
        exporter::export_markdown(md_list_filename, &all_tf_files, &result, false)?;
    }

    // Output the resulting markdown to file as a table
    if let Some(md_table_filename) = cli_args.get_one::<String>("table") {
        exporter::export_markdown(md_table_filename, &all_tf_files, &result, true)?;
    }

    // Export to CSV if filename is given
    if let Some(csv_filename) = cli_args.get_one::<String>("csv") {
        let _exp = exporter::export_csv(csv_filename, &result);
    }

    if !quiet {
        println!("---");
        print_summary(&all_tf_files, &result);
    }

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

/// Build a list of directories to process
///
/// # Arguments
///
/// * `starting_point` - The directory to start from
///
/// # Returns
///
/// * `Vec<String>` - A list of directories to process
fn build_directory_list(starting_point: &str, directory_list: &mut Vec<String>) {
    for entry in WalkDir::new(starting_point)
        .into_iter()
        .filter_map(std::result::Result::ok)
    {
        let path = entry.path();
        if path.is_dir() {
            let directory_path = path.to_string_lossy().to_string();
            directory_list.push(directory_path);
        }
    }
}
