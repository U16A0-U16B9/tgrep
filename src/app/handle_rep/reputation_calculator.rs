use std::collections::HashMap;
use crate::message_data::MessageData;
use crate::services::config::triggers::TriggerType;
use super::Reputations;
use crate::services::persistence_manager::{DataType, PersistenceManager, file_manager::FileManager};

const ADD_STEP: i64 = 1;
const SUB_STEP: i64 = -1;

pub fn calculate_reputation<'a>(data: &'a MessageData, reputations:&'a mut Reputations) -> (i64, &'a  Reputations) {
    let chat_rep = reputations.chats.get(&data.get_chat_id());
    let rep_reset:i64 = 0;

    match chat_rep {
        Some(_chat_rep) => {
            let  user_rep = _chat_rep.get(&data.get_rep_reciv_user_id().unwrap());
            match user_rep {
                Some(_user_rep_points) => {
                    match data.get_trigger_type() {
                        TriggerType::None => {},
                        TriggerType::Positive => {
                            *reputations.chats.entry(data.get_chat_id()).or_default().entry(data.get_rep_reciv_user_id().unwrap()).or_default() += ADD_STEP;
                        },
                        TriggerType::Negative => {
                            *reputations.chats.entry(data.get_chat_id()).or_default().entry(data.get_rep_reciv_user_id().unwrap()).or_default() += SUB_STEP;
                        },
                    }
                },
                None => {
                    match data.get_trigger_type() {
                        TriggerType::None => {},
                        TriggerType::Positive => {
                            *reputations.chats.entry(data.get_chat_id()).or_default().entry(data.get_rep_reciv_user_id().unwrap()).or_default() = ADD_STEP;
                        },
                        TriggerType::Negative => {
                            *reputations.chats.entry(data.get_chat_id()).or_default().entry(data.get_rep_reciv_user_id().unwrap()).or_default() = SUB_STEP;
                        },
                    }
                },
            }
        },
        None => {
            let mut user_rep = HashMap::new();
            user_rep.insert(data.get_rep_reciv_user_id().unwrap(), rep_reset);
            reputations.chats.insert(data.get_chat_id(), user_rep);
        },
    }
    (*reputations.chats.entry(data.get_chat_id()).or_default().entry(data.get_rep_reciv_user_id().unwrap()).or_default(), reputations)
}

pub fn get_reputations() -> Reputations {
    let reputation_text = FileManager::load_data(DataType::ReputationData);
    return match reputation_text {
        Some(_text) => {
            let v: Result<Reputations, serde_json::Error> = serde_json::from_str(_text.as_str());
            match v {
                Ok(_reputations) => _reputations,
                Err(_) =>  Reputations::new(),
            }
        },
        None => Reputations::new(),
    }
}

pub fn save_reputations(reputations: &Reputations) {
    let reputation_text = serde_json::to_string(&reputations);
    match reputation_text {
        Ok(_reputation_text) => {
            FileManager::save_data(DataType::ReputationData, _reputation_text)
        },
        Err(_a) => panic!("{}", _a.to_string()),
    }

}