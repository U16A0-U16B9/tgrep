use std::collections::HashMap;
use crate::app::message_data::MessageData;
use crate::services::config::triggers::TriggerType;
use crate::services::data::reputations::Reputations;

const ADD_STEP: i64 = 1;
const SUB_STEP: i64 = -1;

pub fn calculate_reputation(data: &MessageData, mut reputations: Reputations) -> (i64,  Reputations) {
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
                            reputations = increment_reputation(data, reputations);
                        },
                        TriggerType::Negative => {
                            reputations = decrement_reputation(data, reputations);
                        },
                    }
                },
                None => {
                    match data.get_trigger_type() {
                        TriggerType::None => {},
                        TriggerType::Positive => {
                            reputations = init_reputation(data, reputations, ADD_STEP);
                        },
                        TriggerType::Negative => {
                            reputations = init_reputation(data, reputations, SUB_STEP);
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
    (
        *reputations.chats
            .entry(data.get_chat_id())
            .or_default()
            .entry(data.get_rep_reciv_user_id().unwrap())
            .or_default(),
        reputations
    )
}

fn increment_reputation(data: &MessageData, mut reputations: Reputations) -> Reputations {
    *reputations.chats
        .entry(data.get_chat_id())
        .or_default()
        .entry(data.get_rep_reciv_user_id().unwrap())
        .or_default() += ADD_STEP;

    reputations
}

fn decrement_reputation(data: &MessageData, mut reputations: Reputations) -> Reputations {
    *reputations.chats
        .entry(data.get_chat_id())
        .or_default()
        .entry(data.get_rep_reciv_user_id().unwrap())
        .or_default() += SUB_STEP;

    reputations
}

fn init_reputation(data: &MessageData, mut reputations: Reputations, rep: i64) -> Reputations {
    *reputations.chats
        .entry(data.get_chat_id())
        .or_default()
        .entry(data.get_rep_reciv_user_id().unwrap())
        .or_default() = rep;

    reputations
}