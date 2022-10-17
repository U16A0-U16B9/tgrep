use crate::app::reputation_message::ReputationMessage;
use crate::objects::messages::ParseMessage;
use crate::objects::user::generate_display_name;
use crate::services::config::settings::Settings;
use crate::services::config::triggers::TriggerType;
use crate::services::data::reputations::Reputations;

use super::message_data::MessageData;

mod reputation_calculator;

pub struct HandledReputation {
    pub reciv_username: String,
    pub giver_username: String,
    pub operation: String,
    pub reciv_reputation: i64,
}

impl HandledReputation {
    pub fn handle_rep(reputation_message: &ReputationMessage) -> Option<HandledReputation> {
        if !reputation_message.is_valid {
            return None;
        }

        let operation: String;

        let reputations = Reputations::load();
        let (reciv_reputation, trigger, reputations) = reputation_calculator::calculate_reputation(&reputation_message, reputations);
        let _result = Reputations::save(reputations);

        match trigger {
            TriggerType::None => return None,
            TriggerType::Positive => operation = "increased".to_string(),
            TriggerType::Negative => operation = "decreased".to_string(),
        }

        let reciv_username = reputation_message.rep_reciv
            .as_ref()
            .map(|user| generate_display_name(&user))
            .unwrap_or("Unknown".to_string())
            .clone();
        let giver_username = reputation_message.rep_giver
            .as_ref()
            .map(|user| generate_display_name(&user))
            .unwrap_or("Unknown".to_string())
            .clone();

        let settings = Settings::load();
        if settings.save_history {
            reputation_calculator::save_reputation_to_history(reputation_message, trigger);
        }

        Some(HandledReputation {
            reciv_username,
            giver_username,
            operation,
            reciv_reputation,
        })
    }
}

impl ParseMessage for HandledReputation {
    fn parse(&self) -> String {
        format!(
            "{} has {} reputation of {} to {}",
            self.giver_username, self.operation, self.reciv_username, self.reciv_reputation
        )
    }
}
