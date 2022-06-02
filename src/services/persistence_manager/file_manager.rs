use std::{env, fs};
use std::path::{PathBuf};
use directories::ProjectDirs;
use super::{PersistenceManager, DataType, ConfigType};

const QUALIFIER: &str = "bot";
const ORGANIZATION: &str = "U16A0-U16B9";
const APPLICATION: &str = "tgbot";

const DATA_REPUTATIONS: &str = "reputations.json";

pub struct FileManager {
}

impl PersistenceManager for FileManager {

    fn load_data(data_type: DataType) -> Option<String> {
        let filename= get_data_filename(&data_type);
        let file_text = fs::read_to_string(filename);
        match file_text {
            Ok(_file_text) => { Some(_file_text) }
            Err(_) => { None }
        }
    }

    fn save_data(data_type: DataType, data: String) {
        let filename= get_data_filename(&data_type);
        let result = fs::write(filename, data);
        match result {
            Ok(_ok) => { _ok }
            Err(_err) => { panic!("{}", _err.to_string()) }
        }
    }

    fn load_config(config_type: ConfigType) -> Option<String> {
        todo!()
    }

    fn save_config(config_type: ConfigType, config: String) {
        todo!()
    }
}

fn get_data_filename(data_type: &DataType) -> PathBuf {
    match data_type {
        DataType::ReputationData => {
            get_data_dir().as_path().join(DATA_REPUTATIONS)
        }
    }
}

fn get_data_dir() -> PathBuf {
    let project_dir = get_project_dirs();
    match project_dir {
        None => {get_current_dir()}
        Some(_project_dir) => {_project_dir.data_dir().to_path_buf()}
    }
}

fn get_config_dir() -> PathBuf {
    let project_dir = get_project_dirs();
    match project_dir {
        None => {get_current_dir()}
        Some(_project_dir) => {_project_dir.config_dir().to_path_buf()}
    }
}

fn get_current_dir() -> PathBuf {
    let cd = env::current_dir();
    match cd {
        Ok(_path_buf) => return _path_buf,
        Err(_) => panic!("Cannot read directory"),
    }
}

fn get_project_dirs() -> Option<ProjectDirs> {
    let project_dir = ProjectDirs::from(
        QUALIFIER,
        ORGANIZATION,
        APPLICATION
    );

    match project_dir {
        Some(dir) => {
            let dir_created = fs::create_dir_all(dir.data_dir());
            match dir_created {
                Ok(_) => Some(dir),
                Err(_) => panic!("Cannot read directory"),
            }
        },
        None => None,
    }
}