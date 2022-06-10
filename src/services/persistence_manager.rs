pub mod file_manager;

pub enum DataType {
    ReputationData,
    UserList,
    ReputationHistory,
}

pub enum ConfigType {
    Triggers,
    Settings,
}

pub trait PersistenceManager {
    fn load_data(data_type: DataType) -> Option<String>;
    fn save_data(data_type: DataType, data: String);
    fn load_config(config_type: ConfigType) -> Option<String>;
    fn save_config(config_type: ConfigType, config: String);
}
