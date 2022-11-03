use std::fmt::{Display, Formatter};

pub mod file_manager;

pub enum DataType {
    ReputationData,
    UserList,
    ReputationHistory,
    Chats,
}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::ReputationData => {
                write!(f, "ReputationData")
            }
            DataType::UserList => {
                write!(f, "UserList")
            }
            DataType::ReputationHistory => {
                write!(f, "ReputationHistory")
            }
            DataType::Chats => {
                write!(f, "Chats")
            }
        }
    }
}

pub enum ConfigType {
    Triggers,
    Settings,
}

impl Display for ConfigType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigType::Triggers => {
                write!(f, "Triggers")
            }
            ConfigType::Settings => {
                write!(f, "Settings")
            }
        }
    }
}

pub trait PersistenceManager {
    fn load_data(data_type: DataType) -> Option<String>;
    fn save_data(data_type: DataType, data: String);
    fn load_config(config_type: ConfigType) -> Option<String>;
    fn save_config(config_type: ConfigType, config: String);
}
