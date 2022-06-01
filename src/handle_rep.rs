use serde::{Serialize, Deserialize};
use teloxide::types::{ChatId, UserId};
use crate::message_data::triggers::TriggerType;
use std::collections::HashMap;

use super::MessageData;

mod reputation_calculator;
mod services;

#[derive(Serialize, Deserialize, Debug)]
pub struct Reputations {
    pub chats: HashMap<ChatId, HashMap<UserId, i64>>
}

pub struct HandledReputation {
    pub reciv_username: Option<String>,
    pub giver_username: Option<String>,
    pub operation: String,
    pub reciv_reputation: i64
}

impl HandledReputation {

    pub fn handle_rep(data: &MessageData) -> Option<HandledReputation>  {
        if !data.get_valid() {
            return None;
        }

        let operation: String;

        let mut reputations = reputation_calculator::get_reputations();
        let (reciv_reputation, reputations) = reputation_calculator::calculate_reputation(data, &mut reputations);
        let _result = reputation_calculator::save_reputations(reputations);
        match _result {
            Err(_err) => panic!("{}", _err.to_string()),
            _ => {},
        }

        match data.get_trigger_type() {
            TriggerType::None => return None,
            TriggerType::Positive => operation = "increased".to_string(),
            TriggerType::Negative => operation = "decreased".to_string(),
        }

        Some(HandledReputation {
            reciv_username: data.get_rep_reciv_user_name().clone(),
            giver_username: data.get_rep_giver_user_name().clone(),
            operation,
            reciv_reputation
        })

    }

}