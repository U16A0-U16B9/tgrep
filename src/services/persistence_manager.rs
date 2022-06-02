pub mod file_manager;

pub enum DataType {
    ReputationData
}

pub enum ConfigType {
    Triggers
}

pub trait PersistenceManager{
    fn load_data(data_type:DataType) -> Option<String>;
    fn save_data( data_type:DataType, data: String);
    fn load_config(config_type:ConfigType) -> Option<String>;
    fn save_config(config_type:ConfigType, config: String);
}
