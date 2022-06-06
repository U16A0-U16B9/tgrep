use crate::services::config::triggers::TriggerType;
use crate::services::data::reputations::Reputations;

use super::message_data::MessageData;

mod reputation_calculator;

pub struct HandledReputation {
    pub reciv_username: String,
    pub giver_username: String,
    pub operation: String,
    pub reciv_reputation: i64
}

impl HandledReputation {

    pub fn handle_rep(data: &MessageData) -> Option<HandledReputation>  {
        if !data.get_valid() {
            return None;
        }

        let operation: String;

        let reputations = Reputations::load();
        let (reciv_reputation, reputations) = reputation_calculator::calculate_reputation(&data, reputations);
        let _result = Reputations::save(reputations);

        match data.get_trigger_type() {
            TriggerType::None => return None,
            TriggerType::Positive => operation = "increased".to_string(),
            TriggerType::Negative => operation = "decreased".to_string(),
        }

        let reciv_username = data.get_rep_reciv_user_name()
            .as_ref().unwrap_or(&"Unknown".to_string()).clone();
        let giver_username = data.get_rep_giver_user_name()
            .as_ref().unwrap_or(&"Unknown".to_string()).clone();

        Some(HandledReputation {
            reciv_username,
            giver_username,
            operation,
            reciv_reputation
        })

    }

}
