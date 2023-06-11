//! Utility functions

use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Returns a list of Terraform files based on the file extension `.tf`
///
/// # Errors
/// - Unable to read file
/// - Unable to unwrap entry
pub fn list_tf_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut result: Vec<PathBuf> = vec![];
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let extension = path.extension().unwrap_or_else(|| OsStr::new(""));
        if !path.is_dir() && extension == "tf" {
            result.push(path);
        }
    }
    Ok(result)
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
pub fn build_directory_list(starting_point: &str) -> Vec<String> {
    let mut directory_list: Vec<String> = vec![];

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

    directory_list
}
