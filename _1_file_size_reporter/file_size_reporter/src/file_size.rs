use std::error::Error;
use std::fs;
use std::path::Path;

pub fn get_file_size(path: &Path) -> Result<u64, Box<dyn Error>> {
    let metadata = fs::metadata(path)?;

    if metadata.is_file() {
        Ok(metadata.len())
    } else {
        Err("Provided path is not a file".into())
    }
}