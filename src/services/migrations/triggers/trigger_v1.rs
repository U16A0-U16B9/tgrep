use std::collections::HashMap;
use teloxide::prelude::ChatId;
use crate::services::migrations::triggers::trigger_v0::TriggersV0;
use crate::services::persistence_manager::file_manager::FileManager;
use crate::services::persistence_manager::{ConfigType, PersistenceManager};
use serde::{Deserialize, Serialize};
use crate::services::config::triggers::Triggers;
use crate::services::data::reputations::Reputations;
use crate::services::migrations::triggers::TriggerMigration;

pub const TRIGGER_VERSION: usize = 1;

#[derive(Serialize, Deserialize)]
pub struct TriggersV1 {
    pub version: usize,
    pub chat: HashMap<ChatId, Vec<ChatTriggerV1>>,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct ChatTriggerV1 {
    pub trigger: String,
    pub trigger_type: TriggerTypeV1,
    pub is_wildcard: bool
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum TriggerTypeV1 {
    None,
    Positive,
    Negative,
}

impl TriggersV1 {
    pub fn load() -> Option<TriggersV1> {
        let triggers_text = FileManager::load_config(ConfigType::Triggers);
        match triggers_text {
            None => None,
            Some(_triggers_text) => {
                let triggers_result: Result<TriggersV1, serde_json::Error> =
                    serde_json::from_str(_triggers_text.as_str());
                match triggers_result {
                    Ok(triggers) => Some(triggers),
                    _ => None,
                }
            }
        }
    }
    pub fn migrate(triggers: TriggersV0) -> (Option<TriggersV1>, Option<Triggers>) {
        let mut migrated_triggers = TriggersV1 {
            version: TRIGGER_VERSION,
            chat: HashMap::new()
        };
        let chat_triggers = Self::get_chat_triggers(triggers);
        let chat_ids = Self::get_chat_ids();

        for chat_id in chat_ids.iter() {
           migrated_triggers.chat.insert(*chat_id, chat_triggers.clone());
        }
        let triggers_result = TriggerMigration::convert_to_trigger(&migrated_triggers);

        match triggers_result {
            Ok(triggers) => {
                (None, Some(triggers))
            }
            Err(_) => {
                (Some(migrated_triggers), None)
            }
        }
    }


    fn get_chat_triggers(triggers: TriggersV0) -> Vec<ChatTriggerV1> {
        let mut chat_triggers: Vec<ChatTriggerV1> = vec![];

        for trigger in triggers.positive.iter() {
            chat_triggers.push(ChatTriggerV1{
                trigger: trigger.clone(),
                trigger_type: TriggerTypeV1::Positive,
                is_wildcard: true
            });
        }

        for trigger in triggers.negative.iter() {
            chat_triggers.push(ChatTriggerV1{
                trigger: trigger.clone(),
                trigger_type: TriggerTypeV1::Negative,
                is_wildcard: true
            });
        }
        chat_triggers
    }

    fn get_chat_ids() -> Vec<ChatId> {
        // TODO: use chat list from data
        let reputations = Reputations::load();
        reputations.chats.keys().cloned().collect()
    }
}
