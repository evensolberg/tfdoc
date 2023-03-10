//! Contains a single function to build the main CLI for the `tfdoc` program.
use clap::{command, Arg, Command};

/// Builds the CLI so the main file doesn't get cluttered. Note that the `<'static>` means it returns a global variable.
pub fn build_cli(version: &'static str) -> Command {
    // This is the heading under which all the tags settings are grouped
    // run the app with `-h` to see.
    command!()
        .about("Generates documentation for Terraform modules and deployments.")
        .version(version)
        .author(clap::crate_authors!("\n"))
        .long_about("Generates documentation for Terraform modules and deployments.")
        .override_usage("tfdoc <DIR(S)> [OPTIONS]")
        .arg( // Dirs - the directories to process
            Arg::new("dirs")
                .value_name("DIR(S)")
                .help("One or more directories to process.")
                .long_help("One or more directories to process. Use the ** glob to recurse. Note: Case sensitive.")
                .required(false)
                .default_value(".")
                .num_args(0..)
        )
        .arg( // export as lists
            Arg::new("list")
                .short('l')
                .long("list")
                .help("Output the results as lists in a Markdown file. Default file name: tfdoc_summary_lists.md")
                .num_args(..=1)
                .value_name("FILE")
                .default_missing_value("tfdoc_summary_lists.md")
        )
        .arg( // export as tables
            Arg::new("table")
                .short('t')
                .long("table")
                .help("Output the results as tables in a Markdown file. Default file name: tfdoc_summary_tables.md")
                .num_args(..=1)
                .value_name("FILE")
                .default_missing_value("tfdoc_summary_tables.md")
        )
        .arg( // export to CSV
            Arg::new("csv")
            .short('c')
            .long("csv")
            .help("Output to a CSV file with the name provided. Default file name: tfdoc_summary.csv")
            .long_help("Output to a CSV file with the file name provided.")
            .num_args(..=1)
            .value_name("FILE")
            .default_missing_value("tfdoc_summary.csv")
        )
}
