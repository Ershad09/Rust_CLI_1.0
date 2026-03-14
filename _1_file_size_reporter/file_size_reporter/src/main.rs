

mod args;
mod converter;
mod file_size;

use args::get_file_path;
use file_size::get_file_size;
use converter::convert_size;

use std::process;

fn main() {
    if let Err(err) = report() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}

fn report() -> Result<(), Box<dyn std::error::Error>> {
    let path = get_file_path()?;

    let size = get_file_size(&path)?;

    let (kb, mb, gb) = convert_size(size);

    println!("File: {}", path.display());
    println!("Bytes: {}", size);
    println!("KB: {:.2}", kb);
    println!("MB: {:.2}", mb);
    println!("GB: {:.4}", gb);

    Ok(())
}