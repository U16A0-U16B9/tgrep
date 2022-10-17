use crate::services::persistence_manager::file_manager::FileManager;
use crate::services::persistence_manager::{ConfigType, PersistenceManager};
use log::error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use teloxide::prelude::ChatId;

pub const TRIGGER_VERSION: usize = 1;

#[derive(Serialize, Deserialize)]
pub struct Triggers {
    pub version: usize,
    pub chat: HashMap<ChatId, Vec<ChatTrigger>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatTrigger {
    pub trigger: String,
    pub trigger_type: TriggerType,
    pub is_wildcard: bool,
    pub is_sticker: bool,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum TriggerType {
    None,
    Positive,
    Negative,
}

impl Triggers {
    pub fn new() -> Triggers {
        Triggers {
            version: TRIGGER_VERSION,
            chat: HashMap::new(),
        }
    }

    pub fn load() -> Triggers {
        let triggers_text = FileManager::load_config(ConfigType::Triggers);
        match triggers_text {
            None => Self::save(Self::new()),
            Some(_triggers_text) => {
                let triggers_result: Result<Triggers, serde_json::Error> =
                    serde_json::from_str(_triggers_text.as_str());
                match triggers_result {
                    Ok(_triggers) => _triggers,
                    _ => Self::save(Self::new()),
                }
            }
        }
    }

    pub fn save(triggers: Triggers) -> Triggers {
        let trigger_text = serde_json::to_string(&triggers);
        match trigger_text {
            Ok(_trigger_text) => {
                FileManager::save_config(ConfigType::Triggers, _trigger_text);
            }
            Err(_a) => {
                error!("Cannot save {}", ConfigType::Triggers.to_string());
                panic!("{}", _a.to_string())
            }
        }
        triggers
    }
}

#[cfg(test)]
mod triggers_tests {
    use super::*;

    #[test]
    fn test_new() {
        let triggers = Triggers::new();
        assert_eq!(triggers.positive.len(), 2);
        assert_eq!(triggers.negative.len(), 2);
    }

    #[test]
    fn test_load_save() {
        let rand = String::from("bb3Yzr35ousfdg9ie9Km1jaOJD9Iq15V");
        let mut triggers = Triggers::load();
        triggers.positive.push(rand.clone());
        Triggers::save(triggers);
        let mut triggers = Triggers::load();
        assert!(triggers.positive.contains(&rand));
        // cleanup
        triggers.positive.pop();
        Triggers::save(triggers);
    }
}
