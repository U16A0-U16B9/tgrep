use crate::services::data::chats::{Chat, Chats};
use log::error;
use teloxide::prelude::{AutoSend, Message, Requester};
use teloxide::types::{ChatId, UserId};
use teloxide::Bot;

pub async fn save_chat(bot: &AutoSend<Bot>, message: &Message) {
    let mut chats = Chats::load();
    let chat = message.chat.clone();

    match chats.chats.get(&chat.id) {
        None => {
            let owner = get_owner_id(bot, chat.id).await;
            let chat_data = Chat {
                chat_id: chat.id,
                chat_owner: owner,
            };
            chats.chats.insert(chat.id, chat_data);
            Chats::save(chats);
        }
        Some(_) => {}
    };
}

async fn get_owner_id(bot: &AutoSend<Bot>, chat_id: ChatId) -> UserId {
    let admins_result = bot.get_chat_administrators(chat_id).await;
    let mut owner_id: UserId = UserId(0);
    match admins_result {
        Ok(admins) => {
            for admin in admins.iter() {
                if admin.kind.is_owner() {
                    owner_id = admin.user.id
                }
            }
        }
        Err(err) => {
            error!("Cannot get chat administrators");
            panic!("{}", err.to_string())
        }
    }
    owner_id
}
