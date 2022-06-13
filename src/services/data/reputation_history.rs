use crate::app::message_data::MessageData;
use crate::services::config::triggers::TriggerType;
use crate::services::data::Data;
use crate::services::persistence_manager::file_manager::FileManager;
use crate::services::persistence_manager::{DataType, PersistenceManager};
use log::error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use teloxide::types::{ChatId, MessageId, UserId};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReputationHistory {
    pub chats: HashMap<ChatId, Vec<ReputationHistoryItem>>,
}

#[derive(Serialize, Deserialize, Debug)]
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
    pub fn new(data: &MessageData) -> ReputationHistoryItem {
        ReputationHistoryItem {
            sender: data.get_rep_giver_user_id(),
            receiver: data.get_rep_reciv_user_id(),
            message_id: data.get_message_id(),
            reply_message_id: data.get_reply_message_id(),
            trigger_type: *data.get_trigger_type(),
        }
    }
}

impl Data for ReputationHistory {}
