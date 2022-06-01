use std::{path::PathBuf, env, fs};
use directories::ProjectDirs;

pub fn get_data_dir() -> PathBuf {
    let project_dir = ProjectDirs::from("bot", "U16A0-U16B9", "tgbot");
    match project_dir {
        Some(dir) => {
            let dir_created = fs::create_dir_all(dir.data_dir());
            match dir_created {
                Ok(_) => return dir.data_dir().to_path_buf(),
                Err(_) => panic!("Cannot read directory"),
            }
        },
        None => return get_current_dir(),
    }
}

fn get_current_dir() -> PathBuf {
    let cd = env::current_dir();
    match cd {
        Ok(_path_buf) => return _path_buf,
        Err(_) => panic!("Cannot read directory"),
    }
}
