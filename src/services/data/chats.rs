use crate::services::persistence_manager::file_manager::FileManager;
use crate::services::persistence_manager::{DataType, PersistenceManager};
use log::error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use teloxide::prelude::ChatId;
use teloxide::types::UserId;

pub const CHATS_VERSION: usize = 1;

#[derive(Serialize, Deserialize)]
pub struct Chats {
    pub version: usize,
    pub chats: HashMap<ChatId, Chat>,
}

#[derive(Serialize, Deserialize)]
pub struct Chat {
    pub chat_id: ChatId,
    pub chat_owner: UserId,
}

impl Chats {
    pub fn new() -> Chats {
        Chats {
            version: CHATS_VERSION,
            chats: HashMap::new(),
        }
    }

    pub fn load() -> Chats {
        let chat_text_result = FileManager::load_data(DataType::Chats);
        match chat_text_result {
            None => Chats::new(),
            Some(chat_text) => {
                let chats_result: Result<Chats, serde_json::Error> = serde_json::from_str(chat_text.as_str());
                match chats_result {
                    Ok(chats) => chats,
                    Err(_) => Chats::new(),
                }
            }
        }
    }

    pub fn save(chats: Chats) -> Chats {
        let chat_text_result = serde_json::to_string(&chats);
        match chat_text_result {
            Ok(chat_text) => FileManager::save_data(DataType::Chats, chat_text),
            Err(err) => {
                error!("Cannot save {}", DataType::Chats.to_string());
                panic!("{}", err.to_string())
            }
        }
        chats
    }
}
