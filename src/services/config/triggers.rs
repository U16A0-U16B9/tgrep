use serde::{Serialize, Deserialize};
use crate::services::config::Config;
use crate::services::persistence_manager::{ConfigType, PersistenceManager};
use crate::services::persistence_manager::file_manager::FileManager;

#[derive(Serialize, Deserialize, Debug)]
pub struct Triggers {
    pub positive: Vec<String>,
    pub negative: Vec<String>
}

pub enum TriggerType {
    None,
    Positive,
    Negative
}

impl Config for Triggers {
    fn to_string(&self) -> Option<String> {
        let triggers_json = serde_json::to_string(self);
        match triggers_json {
            Ok(_text) => { Some(_text) }
            Err(_) => { None }
        }
    }
}

impl Triggers {
    pub fn new() -> Triggers {
        Triggers {
            negative: vec![
                String::from("minus"),
                String::from("-"),
            ],
            positive: vec![
                String::from("plus"),
                String::from("+"),
            ]
        }
    }

    pub fn load() -> Triggers {
        let triggers_text = FileManager::load_config(ConfigType::Triggers);
        match triggers_text {
            None => { Self::save(Self::new()) }
            Some(_triggers_text) => {
                let triggers_result: Result<Triggers, serde_json::Error> = serde_json::from_str(_triggers_text.as_str());
                match triggers_result {
                    Ok(_triggers) => {  _triggers }
                    _ => {  Self::save(Self::new()) }
                }
            }
        }
    }

    fn save(triggers: Triggers) -> Triggers {
        let trigger_text = serde_json::to_string(&triggers);
        match trigger_text {
            Ok(_trigger_text) => {
                FileManager::save_config(ConfigType::Triggers, _trigger_text);
            }
            Err(_) => {}
        }
        triggers
    }
}
