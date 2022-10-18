use crate::services::persistence_manager::file_manager::FileManager;
use crate::services::persistence_manager::{ConfigType, PersistenceManager};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TriggersV0 {
    pub positive: Vec<String>,
    pub negative: Vec<String>,
}
impl TriggersV0 {
    pub fn load() -> Option<TriggersV0> {
        let triggers_text = FileManager::load_config(ConfigType::Triggers);
        match triggers_text {
            None => None,
            Some(_triggers_text) => {
                let triggers_result: Result<TriggersV0, serde_json::Error> =
                    serde_json::from_str(_triggers_text.as_str());
                match triggers_result {
                    Ok(triggers) => Some(triggers),
                    _ => None,
                }
            }
        }
    }
}
