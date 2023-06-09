//! Outputs the resulting information in Markdown format, using lists or tables, depending on preference.

use std::path::PathBuf;

use crate::types::{BlockType, DocItem};

/// Outputs a summary of the number of files processed, etc.
///
/// # Arguments
///
/// * `file_list` - The list of files processed
/// * `result` - The list of `DocItem`s
///
/// # Returns
///
/// Nothing.
pub fn print_summary(file_list: &Vec<PathBuf>, result: &[DocItem]) {
    println!("---");
    println!("Files processed:      {:5}", file_list.len());

    let mut data = 0;
    let mut outputs = 0;
    let mut resources = 0;
    let mut variables = 0;

    for item in result {
        match item.category {
            BlockType::Data => {
                data += 1;
            }
            BlockType::Output => outputs += 1,
            BlockType::Resource => resources += 1,
            BlockType::Variable => variables += 1,
            _ => {}
        }
    }

    println!("Data items:           {data:5}");
    println!("Outputs:              {outputs:5}");
    println!("Resources:            {resources:5}");
    println!("Variables:            {variables:5}");
}
