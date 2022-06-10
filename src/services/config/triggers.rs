use crate::services::persistence_manager::file_manager::FileManager;
use crate::services::persistence_manager::{ConfigType, PersistenceManager};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Triggers {
    pub positive: Vec<String>,
    pub negative: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum TriggerType {
    None,
    Positive,
    Negative,
}

impl Triggers {
    pub fn new() -> Triggers {
        Triggers {
            negative: vec![String::from("minus"), String::from("-")],
            positive: vec![String::from("plus"), String::from("+")],
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

    fn save(triggers: Triggers) -> Triggers {
        let trigger_text = serde_json::to_string(&triggers);
        match trigger_text {
            Ok(_trigger_text) => {
                FileManager::save_config(ConfigType::Triggers, _trigger_text);
            }
            Err(_a) => {
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
