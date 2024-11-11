#![windows_subsystem = "windows"]

use std::{env, fs::{self, ReadDir}, path::Path, thread, time::Duration};

use killppp::ConfigKill;

fn main() -> std::io::Result<()> {

    let config = read_config()?;

    let env_path: Vec<String> = env::args().skip(1).collect();
    if env_path.len() <= 0 {
        return Ok(());
    }

    loop {
        for path in &env_path {
            if let Ok(dir) = fs::read_dir(path.clone()) {
                let _ = delete_sub_entry(dir);
            }
        }

        thread::sleep(Duration::from_secs(10));
    }
}

fn read_config() -> std::io::Result<ConfigKill> {
    let config_path = Path::new(env::current_exe()?.parent().unwrap()).with_file_name("config.toml");

    if !fs::exists(config_path.clone())? {
        let config = ConfigKill::default();
        fs::write(config_path, toml::to_string(&config).map_err(|err| std::io::Error::other(err))?)?;
        return Ok(config);
    }

    let read_res = fs::read_to_string(config_path)?;
    
    toml::from_str(&read_res).map_err(|err| std::io::Error::other(err))
}

fn delete_sub_entry(dir: ReadDir) -> std::io::Result<()> {
    for read_entry in dir.into_iter() {
        let entry = read_entry?;
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            delete_dir(&entry.path());
        } else if file_type.is_file() {
            delete_file(&entry.path());
        }
    }
    Ok(())
}

fn delete_file(path: &Path) {
    let _ = fs::remove_file(path);
}

fn delete_dir(path: &Path) {
    let _ = fs::remove_dir_all(path);
}