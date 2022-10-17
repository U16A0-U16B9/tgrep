use crate::app::message_data::MessageData;
use crate::services::config::triggers::{TriggerType};
use crate::services::data::reputation_history::{ReputationHistory, ReputationHistoryItem};
use crate::services::data::reputations::Reputations;
use std::collections::HashMap;
use crate::app::reputation_message::ReputationMessage;

const ADD_STEP: i64 = 1;
const SUB_STEP: i64 = -1;

pub fn calculate_reputation(reputation_message: &ReputationMessage, mut reputations: Reputations) -> (i64, TriggerType, Reputations) {
    let chat_rep = reputations.chats.get(&reputation_message.chat_id);
    let mut trigger_type: TriggerType = TriggerType::None;

    if reputation_message.chat_triggers.iter().any(|trigger|
        trigger.trigger_type == TriggerType::Positive
    ) {
        trigger_type = TriggerType::Positive;
    } else if reputation_message.chat_triggers.iter().any(|trigger|
        trigger.trigger_type == TriggerType::Negative
    ) {
        trigger_type = TriggerType::Negative;
    }

    match chat_rep {
        Some(_chat_rep) => {
            let user_id = reputation_message.rep_reciv.clone().unwrap().id;
            let user_rep = _chat_rep.get(&user_id);
            match user_rep {
                Some(user_rep_points) => {
                    if trigger_type == TriggerType::Positive {
                        reputations = increment_reputation(reputation_message, reputations);
                    } else if trigger_type == TriggerType::Negative {
                        reputations = decrement_reputation(reputation_message, reputations);
                    }
                },
                None => {
                    if trigger_type == TriggerType::Positive {
                        reputations = init_reputation(reputation_message, reputations, ADD_STEP);
                    } else if trigger_type == TriggerType::Negative {
                        reputations = init_reputation(reputation_message, reputations, SUB_STEP);
                    }
                }
            }
        }
        None => {
            let mut user_rep = HashMap::new();
            let user_id = reputation_message.rep_reciv.clone().unwrap().id;
            if trigger_type == TriggerType::Positive {
                user_rep.insert(user_id, ADD_STEP);
            } else if trigger_type == TriggerType::Negative {
                user_rep.insert(user_id, SUB_STEP);
            }
            reputations.chats.insert(reputation_message.chat_id, user_rep);
        }
    }

    (
        *reputations
            .chats
            .entry(reputation_message.chat_id)
            .or_default()
            .entry(reputation_message.rep_reciv.as_ref().unwrap().id)
            .or_default(),
        trigger_type,
        reputations
    )
}

pub fn save_reputation_to_history(reputation_message: &ReputationMessage, trigger_type: TriggerType) {
    let rep_history_item = ReputationHistoryItem::new(reputation_message, trigger_type);
    let mut rep_history = ReputationHistory::load();
    match rep_history.chats.get(&reputation_message.chat_id) {
        None => {
            rep_history.chats.insert(reputation_message.chat_id, vec![rep_history_item]);
        }
        Some(_) => {
            rep_history
                .chats
                .entry(reputation_message.chat_id)
                .or_default()
                .push(rep_history_item);
        }
    }
    ReputationHistory::save(rep_history);
}

fn increment_reputation(reputation_message: &ReputationMessage, mut reputations: Reputations) -> Reputations {
    *reputations
        .chats
        .entry(reputation_message.chat_id)
        .or_default()
        .entry( reputation_message.rep_reciv.as_ref().unwrap().id)
        .or_default() += ADD_STEP;

    reputations
}

fn decrement_reputation(reputation_message: &ReputationMessage, mut reputations: Reputations) -> Reputations {
    *reputations
        .chats
        .entry(reputation_message.chat_id)
        .or_default()
        .entry(reputation_message.rep_reciv.as_ref().unwrap().id)
        .or_default() += SUB_STEP;

    reputations
}

fn init_reputation(reputation_message: &ReputationMessage, mut reputations: Reputations, rep: i64) -> Reputations {
    *reputations
        .chats
        .entry(reputation_message.chat_id)
        .or_default()
        .entry(reputation_message.rep_reciv.as_ref().unwrap().id)
        .or_default() = rep;

    reputations
}
