use std::env;
use std::fs::{copy, create_dir_all, read_dir};
use std::path::{Path, PathBuf};
use std::io;

fn main() {
    let res_dir_source = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("resources/");
    let res_dir_target = Path::new(&env::var("OUT_DIR").unwrap()).join("../../../resources/");

    //copies all resource files to "target/NAME/resources". Prints out any errors if failed.
    if let Err(io_error) = add_resources(&res_dir_source, &res_dir_target) {
        println!("OS Error: {}", io_error);
    }
}

///Recursively copy all files in dir given by source_path to dir given by target path
///WARNING! Overwrites files with same name
fn add_resources(source_path: &PathBuf, target_path: &PathBuf) -> io::Result<()> {
    match read_dir(source_path) {
        Ok(entry_iter) => {
            create_dir_all(target_path)?;
            for entry in entry_iter {
                let entry = entry?;
                let source_path = entry.path();
                let target_path = target_path.join(entry.file_name());
                add_resources(&source_path, &target_path)?;
            }
        }
        Err(_) => {
            copy(&source_path, &target_path)?;
        }
    }
    Ok(())
}
