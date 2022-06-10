use crate::services::data::Data;
use crate::services::persistence_manager::file_manager::FileManager;
use crate::services::persistence_manager::{DataType, PersistenceManager};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use teloxide::types::{ChatId, UserId};

#[derive(Serialize, Deserialize, Debug)]
pub struct Reputations {
    pub chats: HashMap<ChatId, HashMap<UserId, i64>>,
}

impl Reputations {
    pub fn new() -> Reputations {
        Reputations { chats: HashMap::new() }
    }

    pub fn load() -> Reputations {
        let reputation_text = FileManager::load_data(DataType::ReputationData);
        return match reputation_text {
            Some(_text) => {
                let v: Result<Reputations, serde_json::Error> = serde_json::from_str(_text.as_str());
                match v {
                    Ok(_reputations) => _reputations,
                    Err(_) => Reputations::new(),
                }
            }
            None => Reputations::new(),
        };
    }

    pub fn save(reputations: Reputations) -> Reputations {
        let reputation_text = serde_json::to_string(&reputations);
        match reputation_text {
            Ok(_reputation_text) => FileManager::save_data(DataType::ReputationData, _reputation_text),
            Err(_a) => panic!("{}", _a.to_string()),
        }
        reputations
    }
}

impl Data for Reputations {}

#[cfg(test)]
mod reputation_tests {
    use super::*;

    #[test]
    fn test_new() {
        let reputations = Reputations::new();
        assert_eq!(reputations.chats.len(), 0)
    }

    #[test]
    fn test_load_save() {
        let chat_id = ChatId(0xAA);
        let user_id = UserId(17);
        let rep: i64 = 1337;
        let mut reputations = Reputations::load();

        let mut user_rep = HashMap::new();
        user_rep.insert(user_id, rep);
        reputations.chats.insert(chat_id, user_rep);

        Reputations::save(reputations);
        let mut reputations = Reputations::load();

        assert!(reputations.chats.contains_key(&chat_id));
        assert!(reputations.chats[&chat_id].contains_key(&user_id));
        assert_eq!(reputations.chats[&chat_id][&user_id], rep);

        //cleanup
        reputations.chats.remove(&chat_id);
    }
}
