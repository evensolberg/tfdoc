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

    write!(export_file, "Filename,Category,Type,Name,Description\n")?;

    for item in result {
        if item.category != BlockType::Comment {
            log::trace!("item = {:?}", item);
            let mut long_desc = String::new();

            for desc in &item.description {
                long_desc = format!("{} {}", long_desc, desc);
                long_desc = long_desc.trim().to_string();
            }

            // Data and Resources we split into type and name
            // TODO: Create separate items in the original struct and handle it properly
            if item.category == BlockType::Data || item.category == BlockType::Resource {
                let type_name: Vec<&str> = item.name.split(".").collect();
                log::trace!("name_split = {:?}", type_name);
                write!(
                    export_file,
                    "{},{},{},{},\"{}\"\n",
                    item.filename, item.category, type_name[0], type_name[1], long_desc
                )?;
            } else { // Ignore the type
                write!(
                    export_file,
                    "{},{},{},{},\"{}\"\n",
                    item.filename, item.category, "", item.name, long_desc
                )?;
            }

        }
    }

    Ok(())
}
