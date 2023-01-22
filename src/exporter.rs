//! Implements exporters

use std::fs::File;
use std::io::Write;

use crate::types::{BlockType, DocItem};

/// Exports results to CSV
pub fn export_csv(
    filename: &str,
    result: &[DocItem],
) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let mut export_file = File::create(filename)?;

    write!(export_file, "Filename,Type,Name,Description\n")?;

    for item in result {
        if item.category != BlockType::Comment {
            let mut long_desc = String::new();
            for desc in &item.description {
                long_desc = format!("{} {}", long_desc, desc);
                long_desc = long_desc.trim().to_string();
            }
            write!(
                export_file,
                "{},{},{},\"{}\"\n",
                item.filename, item.category, item.name, long_desc
            )?;
        }
    }

    Ok(())
}
