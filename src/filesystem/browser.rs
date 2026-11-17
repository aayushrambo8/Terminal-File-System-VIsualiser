use std::fs;
use std::path::{Path, PathBuf};

use super::FileEntry;

pub fn load_directory(path: &Path) -> std::io::Result<Vec<FileEntry>> {
    let mut entries = Vec::new();
    if !path.exists() {
        return Ok(entries);
    }
    let mut dir_entries = fs::read_dir(path)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .collect::<Vec<PathBuf>>();
    dir_entries.sort();
    for item in dir_entries {
        let entry = FileEntry::from_path(item);
        entries.push(entry);
    }
    Ok(entries)
}
