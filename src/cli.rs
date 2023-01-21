//! Contains a single function to build the main CLI for the `tfdoc` program.
use clap::{command, Arg, Command};

/// Builds the CLI so the main file doesn't get cluttered. Note that the `<'static>` means it returns a global variable.
pub fn build_cli(version: &'static str) -> Command {
    // This is the heading under which all the tags settings are grouped
    // run the app with `-h` to see.
    let cmd = command!()
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
        )
        .arg( // Stop on error
            Arg::new("stop-on-error")
                .short('s')
                .long("stop-on-error")
                .help("Stop on error.")
                .long_help("Stop on error. If this flag isn't set, the application will attempt to continue in case of error.")
                // .takes_values(false)
        )
        .arg( // Dry-run
            Arg::new("dry-run")
                .short('r')
                .long("dry-run")
                .help("Iterate through the files and produce output without actually processing anything.")
        )
        .arg( // Print summary information
            Arg::new("print-summary")
                .short('p')
                .long("print-summary")
                .help("Print summary after all files are processed.")
        )
        .arg( // Don't export detail information
            Arg::new("detail-off")
                .short('o')
                .long("detail-off")
                .help("Don't display detailed information about each file processed.")
        )
        .arg( // export as a table
            Arg::new("table")
                .short('t')
                .long("table")
                .help("Output the results as a table (rather than a list).")
                .action(clap::ArgAction::SetTrue)
        );
    // return the cmd
    cmd
}
