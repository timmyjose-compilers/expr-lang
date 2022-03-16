use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn get_all_test_files<P: AsRef<Path>>(dir: &P, files: &mut Vec<PathBuf>) -> io::Result<()> {
    if dir.as_ref().is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                get_all_test_files(&path, files)?;
            } else {
                files.push(path.to_path_buf());
            }
        }
    }

    Ok(())
}
