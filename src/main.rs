use color_eyre::{eyre::Context, Result};
use std::{fs::File, io::BufReader, path::Path};

fn check_file_ext(file_path: String) -> Result<()> {
    if !file_path.ends_with(".json") && !file_path.ends_with(".arb") {
        return Err(color_eyre::eyre::eyre!(
            "file name must end with .json or .arb"
        ));
    }
    Ok(())
}

fn check_file_exist<P: AsRef<Path>>(file_path: P) -> Result<()> {
    let is_exist = file_path
        .as_ref()
        .try_exists()
        .wrap_err("file permission related issue")?;

    let is_file = file_path.as_ref().is_file();

    if !is_exist {
        return Err(color_eyre::eyre::eyre!("file does not exist"));
    }

    if !is_file {
        return Err(color_eyre::eyre::eyre!("given path is not a file"));
    }

    Ok(())
}

fn read_files<P: AsRef<Path>>(file_path: P) -> Result<()> {
    let file = File::open(file_path).wrap_err("file permission related issue")?;
    let reader = BufReader::new(file);

    Ok(())
}

fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}
