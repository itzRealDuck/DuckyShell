use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn walk_dir(dir: &Path, pattern: &str) -> io::Result<()> {
    if dir.is_dir() {
        let entries = fs::read_dir(dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_dir(&path, pattern)?; // Recursively call for subdirectories
            } else {
                let filename = path.file_name().unwrap().to_str().unwrap().to_lowercase();
                if filename.contains(&pattern.to_lowercase()) {
                    // Process the file here (e.g., print filename)
                    println!("{}", path.display());
                }
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let start_dir = Path::new("/home/itzreakduck"); // Replace with your starting path
    let pattern = "main.rs"; // Replace with your filename pattern (case-insensitive)
    walk_dir(start_dir, pattern)
}
