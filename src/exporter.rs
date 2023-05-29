//! Implements exporters

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::{fs::File, path::PathBuf};

use crate::types::{BlockType, DocItem};

/// Exports results to CSV
///
/// # Arguments
///
/// * `filename` - The name of the file to export to
/// * `result` - The results to export
///
/// # Returns
///
/// * `Ok(())` - If the export was successful
/// * `Err(Box<dyn std::error::Error>)` - If the export failed
///
/// # Example
///
/// ```ignore
/// let result = tfdoc::process_dirs(&dirs, &settings);
/// tfdoc::exporter::export_csv("results.csv", &result);
/// ```
pub fn export_csv(
    filename: &str,
    result: &[DocItem],
) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let full_filename = shellexpand::full(filename)?.to_string();
    let mut ef = File::create(full_filename)?;

    writeln!(ef, "Filename,Category,Type,Name,Description")?;

    // Write each item
    for item in result {
        if item.category != BlockType::Comment {
            // Some items have more than one description line,
            // so we collect them into a single string
            let mut long_desc = String::new();
            for desc in &item.description {
                long_desc = format!("{long_desc} {desc}");
                long_desc = long_desc.trim().to_string();
            }

            // Data and Resources we split into type and name
            // TODO: Create separate items in the original struct and handle it properly there
            if !item.filename.is_empty() {
                if item.category == BlockType::Data || item.category == BlockType::Resource {
                    let type_name: Vec<&str> = item.name.split('.').collect();
                    writeln!(
                        ef,
                        "{},{},{},{},\"{long_desc}\"",
                        item.filename, item.category, type_name[0], type_name[1]
                    )?;
                } else {
                    // Ignore the type
                    writeln!(
                        ef,
                        "{},{},,{},\"{long_desc}\"",
                        item.filename, item.category, item.name
                    )?;
                }
            }
        }
    }

    Ok(())
}

/// Exports the results to a markdown file
///
/// # Arguments
///
/// * `md_file` - The name of the markdown file to export to
/// * `file_list` - The list of files to include in the markdown file
/// * `result` - The results to export
/// * `as_table` - Whether to export as a table or not
///
/// # Returns
///
/// * `Ok(())` - If the export was successful
/// * `Err(Box<dyn std::error::Error>)` - If the export failed
pub fn export_markdown(
    md_file: &str,
    file_list: &Vec<PathBuf>,
    result: &[DocItem],
    as_table: bool,
) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    // Create the file
    let full_filename = shellexpand::full(md_file)?.to_string();
    let _f = File::create(Path::new(&full_filename))?;

    // Print the H1 Title: block
    for item in result.iter().filter(|i| i.category == BlockType::Comment) {
        export_title_block(&full_filename, &item.description)?;
    }

    // Print the H2 Blocks
    export_resources(
        &full_filename,
        result,
        "Resource",
        BlockType::Resource,
        as_table,
    )?;
    export_resources(&full_filename, result, "Data", BlockType::Data, as_table)?;
    export_interfaces(
        &full_filename,
        result,
        "Input",
        BlockType::Variable,
        as_table,
    )?;
    export_interfaces(
        &full_filename,
        result,
        "Output",
        BlockType::Output,
        as_table,
    )?;

    export_file_list(&full_filename, file_list, as_table)?;

    Ok(())
}

/// Exports the H1 title block to Markdown
fn export_title_block(
    filename: &str,
    description: &[String],
) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let full_filename = shellexpand::full(filename)?.to_string();
    let mut ef = OpenOptions::new()
        .write(true)
        .append(true)
        .open(Path::new(&full_filename))?;

    let blank_string = String::new();
    let title = &description.first().unwrap_or(&blank_string)["Title: ".len()..];
    writeln!(ef, "# {title}\n")?;

    for line in description.iter().skip(1) {
        writeln!(ef, "{line}")?;
    }

    Ok(())
}

/// Export resources to a Markdown file
///
/// # Arguments
///
/// * `filename` - The name of the file to export to
/// * `result` - The results to export
/// * `name` - The name of the block
/// * `variant` - The type of block
/// * `as_table` - Whether to export as a table or not
///
/// # Returns
///
/// * `Ok(())` - If the export was successful
/// * `Err(Box<dyn std::error::Error>)` - If the export failed
fn export_resources(
    filename: &str,
    result: &[DocItem],
    name: &str,
    variant: BlockType,
    as_table: bool,
) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let full_filename = shellexpand::full(filename)?.to_string();
    let mut ef = OpenOptions::new()
        .write(true)
        .append(true)
        .open(Path::new(&full_filename))?;

    for (index, item) in result.iter().filter(|i| i.category == variant).enumerate() {
        if index == 0 {
            if variant == BlockType::Data {
                writeln!(ef, "\n## {name}\n")?;
            } else {
                writeln!(ef, "\n## {name}s\n")?;
            }

            if as_table {
                writeln!(ef, "|Filename|{name}|Description|")?;
                writeln!(ef, "|-----|---------|")?;
            }
        }

        if !item.filename.is_empty() {
            if as_table {
                writeln!(
                    ef,
                    "|{}|`{}`|{}|",
                    item.filename,
                    item.name,
                    item.description.join(" ")
                )?;
            } else {
                writeln!(
                    ef,
                    "* {} : `{}`: {}",
                    item.filename,
                    item.name,
                    item.description.join(" ")
                )?;
            }
        }
    }

    Ok(())
}

/// Exports the interfaces (ie. the `variable` and `output` sections) to Markdown
///
/// # Arguments
///
/// * `filename` - The name of the file to export to
/// * `result` - The results to export
/// * `name` - The name of the block
/// * `variant` - The type of block
/// * `as_table` - Whether to export as a table or not
///
/// # Returns
///
/// * `Ok(())` - If the export was successful
/// * `Err(Box<dyn std::error::Error>)` - If the export failed
fn export_interfaces(
    filename: &str,
    result: &[DocItem],
    name: &str,
    variant: BlockType,
    as_table: bool,
) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let full_filename = shellexpand::full(filename)?.to_string();
    let mut ef = OpenOptions::new()
        .write(true)
        .append(true)
        .open(Path::new(&full_filename))?;

    for (index, item) in result.iter().filter(|i| i.category == variant).enumerate() {
        if index == 0 {
            writeln!(ef, "\n## {name}s\n")?;

            if as_table {
                writeln!(ef, "|{name}|Description|")?;
                writeln!(ef, "|-----|---------|")?;
            }
        }

        // print the main body
        if as_table {
            if item.description.is_empty() {
                writeln!(ef, "|`{}`||", item.name)?;
            } else {
                writeln!(ef, "|`{}`|{}|", item.name, item.description.join(" "))?;
            }
        } else if item.description.is_empty() {
            writeln!(ef, "* `{}`", item.name)?;
        } else {
            writeln!(ef, "* `{}`: {}", item.name, item.description.join(" "))?;
        }
    }

    Ok(())
}

/// Exports the file list to Markdown
///
/// # Arguments
///
/// * `filename` - The name of the file to export to
/// * `file_list` - The list of files to export
/// * `table` - Whether to export as a table or not
///
/// # Returns
///
/// * `Ok(())` - If the export was successful
/// * `Err(Box<dyn std::error::Error>)` - If the export failed
fn export_file_list(
    filename: &str,
    file_list: &Vec<PathBuf>,
    table: bool,
) -> Result<(), Box<(dyn std::error::Error)>> {
    let full_filename = shellexpand::full(filename)?.to_string();
    let mut ef = OpenOptions::new()
        .write(true)
        .append(true)
        .open(Path::new(&full_filename))?;

    writeln!(ef, "\n## Files\n")?;
    if table {
        writeln!(ef, "|File Name|Description|")?;
        writeln!(ef, "|-----|---------|")?;
    }

    for file in file_list {
        if table {
            writeln!(ef, "|`{}`||", &file.to_str().unwrap_or("Unknown"))?;
        } else {
            writeln!(ef, "* `{}`", &file.to_str().unwrap_or("Unknown"))?;
        }
    }

    Ok(())
}
