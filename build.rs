use std::{collections::BTreeSet, fs, path::PathBuf};

use checksums::{hash_file, Algorithm};
use dirs::config_dir;
use walkdir::WalkDir;

fn get_files(path: &PathBuf) -> BTreeSet<(PathBuf, String)> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            let parent = path;
            let path = e.path().to_path_buf();
            let hash = hash_file(&path, Algorithm::MD5);
            (path.strip_prefix(parent).unwrap().to_path_buf(), hash)
        })
        .collect()
}

use std::io;

pub fn create_dir_for_path(path: &PathBuf) -> io::Result<()> {
    if !path.exists() {
        match create_dir_for_path(&path.parent().unwrap().to_path_buf()) {
            Ok(_) => fs::create_dir(path),
            Err(e) => Err(e),
        }
    } else {
        Ok(())
    }
}

fn main() {
    println!("cargo:rerun-if-changed=includes"); // <- uncomment if ready to use
    let our = get_files(&PathBuf::from("includes"));

    let mut path = config_dir().unwrap();

    path.push(env!("CARGO_PKG_NAME"));
    path.push(PathBuf::from("includes"));

    if let Err(e) = create_dir_for_path(&path) {
        println!("cargo:warning={:?}", e);
    }

    let target = get_files(&path);

    let for_copy = our.difference(&target).map(|item| item.0.clone());

    for item in for_copy {
        let mut src = PathBuf::from("includes");
        src.push(item.clone());
        let mut dst = path.clone();
        dst.push(item.clone());

        if let Err(e) = create_dir_for_path(&dst.parent().unwrap().to_path_buf()) {
            println!(
                "cargo:warning=Failed to create directory for {:?}, {:?}",
                dst, e
            );
        }

        if let Err(e) = fs::copy(src, dst) {
            println!("cargo:warning=Copy failed with error: {:?}", e);
        }
    }
}
