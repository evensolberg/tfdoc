//! Outputs the resulting information in Markdown format, using lists or tables, depending on preference.

use std::path::PathBuf;

use crate::types::{BlockType, DocItem};

/// Renders the results to a markdown file
pub fn render(result: &[DocItem], as_table: bool) {
    // Print the H1 Title: block
    for item in result.iter().filter(|i| i.category == BlockType::Comment) {
        print_title_block(&item.description);
    }

    // Print the H2 Blocks
    print_resources(result, "Resource", BlockType::Resource, as_table);
    print_resources(result, "Data", BlockType::Data, as_table);
    print_interfaces(result, "Input", BlockType::Variable, as_table);
    print_interfaces(result, "Output", BlockType::Output, as_table);
}

/// Creates the H1 title block
fn print_title_block(description: &[String]) {
    let blank_string = String::new();
    let title = &description.first().unwrap_or(&blank_string)["Title: ".len()..];
    println!("# {title}\n");
    for line in description.iter().skip(1) {
        println!("{line}");
    }
}

/// Outputs the `resource` items
fn print_resources(result: &[DocItem], name: &str, variant: BlockType, as_table: bool) {
    log::debug!("print_resources::result = {:?}", result);
    for (index, item) in result.iter().filter(|i| i.category == variant).enumerate() {
        if index == 0 {
            if variant == BlockType::Data {
                println!("\n## {name}\n");
            } else {
                println!("\n## {name}s\n");
            }
            if as_table {
                println!("|{name}|Description|\n|-----|---------|");
            }
        }

        if as_table {
            println!("|`{}`|{}|", item.name, item.description.join(" "));
        } else {
            println!("* `{}`: {}", item.name, item.description.join(" "));
        }
    }
}

/// Outputs the interfaces (ie. the `variable` and `output` sections)
fn print_interfaces(result: &[DocItem], name: &str, variant: BlockType, as_table: bool) {
    for (index, item) in result.iter().filter(|i| i.category == variant).enumerate() {
        if index == 0 {
            println!("\n## {name}s\n");
            if as_table {
                println!("|{name}|Description|\n|-----|---------|");
            }
        }

        // print the main body
        if as_table {
            if item.description.is_empty() {
                println!("|`{}`||", item.name);
            } else {
                println!("|`{}`|{}|", item.name, item.description.join(" "));
            }
        } else {
            if item.description.is_empty() {
                println!("* `{}`", item.name);
            } else {
                println!("* `{}`: {}", item.name, item.description.join(" "));
            }
        }
    }
}

/// Outputs the file list
pub fn print_files(files: &Vec<PathBuf>, table: bool) {
    println!("\n## Files\n");
    if table {
        println!("|File Name|Description|\n|-----|---------|");
    }
    for file in files {
        if table {
            println!("|`{}`||", &file.to_str().unwrap_or("Unknown"));
        } else {
            println!("* `{}`", &file.to_str().unwrap_or("Unknown"));
        }
    }
}
