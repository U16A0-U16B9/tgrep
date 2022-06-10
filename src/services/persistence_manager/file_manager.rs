use super::{ConfigType, DataType, PersistenceManager};
use directories::ProjectDirs;
use std::path::PathBuf;
use std::{env, fs};

const QUALIFIER: &str = "bot";
const ORGANIZATION: &str = "U16A0-U16B9";
const APPLICATION: &str = "tgbot";

const DATA_REPUTATIONS: &str = "reputations.json";
const DATA_USER_LIST: &str = "user-list.json";
const DATA_REPUTATION_HISTORY: &str = "reputation-history.json";

const CONFIG_TRIGGERS: &str = "triggers.json";
const CONFIG_SETTINGS: &str = "settings.json";

pub struct FileManager {}

impl PersistenceManager for FileManager {
    fn load_data(data_type: DataType) -> Option<String> {
        let filename = get_data_filename(&data_type);
        let file_text = fs::read_to_string(filename);
        match file_text {
            Ok(_file_text) => Some(_file_text),
            Err(_) => None,
        }
    }

    fn save_data(data_type: DataType, data: String) {
        let filename = get_data_filename(&data_type);
        let result = fs::write(filename, data);
        match result {
            Ok(_ok) => _ok,
            Err(_err) => {
                panic!("{}", _err.to_string())
            }
        }
    }

    fn load_config(config_type: ConfigType) -> Option<String> {
        let filename = get_config_filename(&config_type);
        let file_text = fs::read_to_string(filename);
        match file_text {
            Ok(_file_text) => Some(_file_text),
            Err(_) => None,
        }
    }

    fn save_config(config_type: ConfigType, config: String) {
        let filename = get_config_filename(&config_type);
        let result = fs::write(filename, config);
        match result {
            Ok(_ok) => _ok,
            Err(_err) => {
                panic!("{}", _err.to_string())
            }
        }
    }
}

fn get_data_filename(data_type: &DataType) -> PathBuf {
    match data_type {
        DataType::ReputationData => get_data_dir().as_path().join(DATA_REPUTATIONS),
        DataType::UserList => get_data_dir().as_path().join(DATA_USER_LIST),
        DataType::ReputationHistory => get_data_dir().as_path().join(DATA_REPUTATION_HISTORY),
    }
}

fn get_config_filename(config_type: &ConfigType) -> PathBuf {
    match config_type {
        ConfigType::Triggers => get_config_dir().as_path().join(CONFIG_TRIGGERS),
        ConfigType::Settings => get_config_dir().as_path().join(CONFIG_SETTINGS),
    }
}

fn get_data_dir() -> PathBuf {
    let project_dir = get_project_dirs();
    match project_dir {
        None => get_current_dir(),
        Some(_project_dir) => _project_dir.data_dir().to_path_buf(),
    }
}

fn get_config_dir() -> PathBuf {
    let project_dir = get_project_dirs();
    match project_dir {
        None => get_current_dir(),
        Some(_project_dir) => _project_dir.config_dir().to_path_buf(),
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
    let project_dir = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION);

    match project_dir {
        Some(dir) => {
            let data_dir_created = fs::create_dir_all(dir.data_dir());
            let config_dir_created = fs::create_dir_all(dir.config_dir());
            match (data_dir_created, config_dir_created) {
                (Ok(_), Ok(_)) => Some(dir),
                _ => panic!("Cannot read directory"),
            }
        }
        None => None,
    }
}

#[cfg(test)]
mod file_manager_tests {
    use super::*;
    use crate::services::config::triggers::Triggers;
    use crate::services::data::reputations::Reputations;

    #[test]
    fn test_get_project_dirs() {
        get_project_dirs().unwrap();
    }

    #[test]
    fn test_get_current_dirs() {
        get_current_dir();
    }

    #[test]
    fn test_get_data_filename() {
        let rep_filename = get_data_filename(&DataType::ReputationData);
        assert!(rep_filename.file_name().unwrap().to_str().unwrap().eq(DATA_REPUTATIONS));
    }

    #[test]
    fn test_get_config_filename() {
        let rep_filename = get_config_filename(&ConfigType::Triggers);
        assert!(rep_filename.file_name().unwrap().to_str().unwrap().eq(CONFIG_TRIGGERS));

        let rep_filename = get_config_filename(&ConfigType::Settings);
        assert!(rep_filename.file_name().unwrap().to_str().unwrap().eq(CONFIG_SETTINGS));
    }

    #[test]
    fn test_load_data() {
        let rep = FileManager::load_data(DataType::ReputationData).unwrap();

        let _: Reputations = serde_json::from_str(&rep).unwrap();
    }

    #[test]
    fn test_load_config() {
        let trigger = FileManager::load_config(ConfigType::Triggers).unwrap();

        let _: Triggers = serde_json::from_str(&trigger).unwrap();
    }

    #[test]
    fn test_save_data() {
        let rep = FileManager::load_data(DataType::ReputationData).unwrap();

        FileManager::save_data(DataType::ReputationData, rep);
    }

    #[test]
    fn test_save_config() {
        let trigger = FileManager::load_config(ConfigType::Triggers).unwrap();

        FileManager::save_config(ConfigType::Triggers, trigger);
    }
}
