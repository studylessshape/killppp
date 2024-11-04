#![windows_subsystem = "windows"]

use std::{env, fs, path::Path, thread, time::Duration};

fn main() {
    loop {
        let env_path: Vec<String> = env::args().skip(1).collect();
        for path in env_path {
            if let Ok(dir) = fs::read_dir(path.clone()) {
                for read_entry in dir.into_iter() {
                    if let Ok(entry) = read_entry {
                        if let Ok(file_type) = entry.file_type() {
                            if file_type.is_dir() {
                                delete_dir(&entry.path());
                            } else if file_type.is_file() {
                                delete_file(&entry.path());
                            }
                        }
                    }
                }
            }
        }

        thread::sleep(Duration::from_secs(10));
    }
}

fn delete_file(path: &Path) {
    let _ = fs::remove_file(path);
}

fn delete_dir(path: &Path) {
    let _ = fs::remove_dir_all(path);
}