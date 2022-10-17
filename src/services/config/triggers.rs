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
        assert_eq!(triggers.version, TRIGGER_VERSION);
        assert_eq!(triggers.chat.iter().count(), 0)
    }

    #[test]
    fn test_load_save() {
        let rand = "bb3Yzr35ousfdg9ie9Km1jaOJD9Iq15V";
        let mut triggers = Triggers::load();
        let chat_id = ChatId(0xBB);
        let chat_trigger = ChatTrigger {
            trigger: rand.to_string(),
            trigger_type: TriggerType::Positive,
            is_wildcard: true,
            is_sticker: false,
        };
        triggers.chat.insert(chat_id, vec![chat_trigger]);
        Triggers::save(triggers);
        let mut triggers = Triggers::load();
        let trigger_string = triggers.chat.get(&chat_id).unwrap().get(0).unwrap().clone().trigger;
        assert!(trigger_string.eq(&rand.to_string()));
        // cleanup
        triggers.chat.remove(&chat_id);
        Triggers::save(triggers);
    }
}
