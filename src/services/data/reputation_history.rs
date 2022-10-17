use crate::services::config::triggers::TriggerType;
use crate::services::data::Data;
use crate::services::persistence_manager::file_manager::FileManager;
use crate::services::persistence_manager::{DataType, PersistenceManager};
use log::error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use teloxide::types::{ChatId, MessageId, UserId};
use crate::app::reputation_message::ReputationMessage;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReputationHistory {
    pub chats: HashMap<ChatId, Vec<ReputationHistoryItem>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReputationHistoryItem {
    pub sender: Option<UserId>,
    pub receiver: Option<UserId>,
    pub message_id: MessageId,
    pub reply_message_id: Option<MessageId>,
    pub trigger_type: TriggerType,
}

impl ReputationHistory {
    pub fn new() -> ReputationHistory {
        ReputationHistory { chats: HashMap::new() }
    }

    pub fn load() -> ReputationHistory {
        let reputation_history_text = FileManager::load_data(DataType::ReputationHistory);
        return match reputation_history_text {
            Some(_text) => {
                let v: Result<ReputationHistory, serde_json::Error> = serde_json::from_str(_text.as_str());
                match v {
                    Ok(_reputations) => _reputations,
                    Err(_) => ReputationHistory::new(),
                }
            }
            None => ReputationHistory::new(),
        };
    }

    pub fn save(reputations: ReputationHistory) -> ReputationHistory {
        let reputation_history_text = serde_json::to_string(&reputations);
        match reputation_history_text {
            Ok(_reputation_history_text) => {
                FileManager::save_data(DataType::ReputationHistory, _reputation_history_text)
            }
            Err(err) => {
                error!("Cannot save {}", DataType::ReputationHistory.to_string());
                panic!("{}", err.to_string())
            }
        }
        reputations
    }
}

impl ReputationHistoryItem {
    pub fn new(reputation_message: &ReputationMessage,  trigger_type: TriggerType) -> ReputationHistoryItem {
        ReputationHistoryItem {
            sender: reputation_message.rep_giver.as_ref().map(|user| user.id),
            receiver: reputation_message.rep_reciv.as_ref().map(|user| user.id),
            message_id: reputation_message.message_id,
            reply_message_id: reputation_message.reply_message.as_ref().map(|message| MessageId { message_id: message.id }),
            trigger_type,
        }
    }
}

impl PartialEq for ReputationHistoryItem {
    fn eq(&self, other: &Self) -> bool {
        self.message_id == other.message_id && self.reply_message_id == other.reply_message_id
    }
}

impl Data for ReputationHistory {}

#[cfg(test)]
mod reputation_history_tests {
    use super::*;

    #[test]
    fn test_new() {
        let reputations = ReputationHistory::new();
        assert_eq!(reputations.chats.len(), 0)
    }

    #[test]
    fn test_load_save() {
        let chat_id = ChatId(0xAA);
        let reputation_history_item = ReputationHistoryItem {
            sender: Some(UserId(17)),
            receiver: Some(UserId(33)),
            message_id: MessageId { message_id: 404 },
            reply_message_id: Some(MessageId { message_id: 403 }),
            trigger_type: TriggerType::Positive,
        };

        let mut reputation_history = ReputationHistory::load();
        reputation_history.chats.insert(chat_id, vec![reputation_history_item]);

        ReputationHistory::save(reputation_history);
        let mut reputation_history = ReputationHistory::load();

        assert!(reputation_history.chats.contains_key(&chat_id));
        assert!(reputation_history.chats[&chat_id]
            .iter()
            .any(|&i| i == reputation_history_item));

        // cleanup
        reputation_history.chats.remove(&chat_id);
    }
}
