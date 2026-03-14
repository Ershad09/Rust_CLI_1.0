use std::env;
use std::error::Error;
use std::path::PathBuf;

pub fn get_file_path() -> Result<PathBuf, Box<dyn Error>> {
    let args = env::args().nth(1);
    
    match args{
        Some(path) => Ok(PathBuf::from(path)),
        None => Err("Please provide a file path".into()),
    }
}