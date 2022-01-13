use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};

fn fetch_files_path(path: &Path, extension: &str, recursive: bool, path_vec: &mut Vec<PathBuf>) -> Result<()> {
    let path = Path::new(path);
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if recursive {
                if let Err(e) = fetch_files_path(&path, extension, recursive, path_vec) {
                    println!("Error: {:?}", e);
                    std::process::exit(1);
                }
            }
        } else if let Some(ext) = path.extension(){
            if ext.to_str() == Some(extension){
                if let Some(file_name) = path.file_name(){
                    if let Some(file_name_str) = file_name.to_str() {
                        if !file_name_str.starts_with("~") {
                            path_vec.push(path);
                        }else{
                            println!("IGNORE: {:?}", file_name);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn get_all_files(path: &str, extension: &str, recursive: bool) -> Vec<PathBuf> {
    let path = Path::new(path);
    let mut path_vec: Vec<PathBuf> = Vec::new();
    if let Err(e) = fetch_files_path(&path, extension, recursive, &mut path_vec) {
        println!("Error: {:?}", e);
        std::process::exit(1);
    };

    path_vec
}

// pub fn execute() -> Result<()> {
//     let path = Path::new("D:/Projects/Rust/data");
//     let mut path_vec: Vec<PathBuf> = Vec::new();
//     if let Err(e) = fetch_files_path(&path, false, &mut path_vec) {
//         println!("Error: {:?}", e);
//         std::process::exit(1);
//     }
//     // for path in path_vec {
//     //     println!("{:?}", path);

//     // }
//     let path = &path_vec[0];
//     println!("{:?}", path);

//     let mut file = fs::File::open(path)?;
//     let mut contents = vec![];
//     file.read_to_end(&mut contents)?;
//     println!("{}", contents.len());

//     Ok(())
// }
