use teloxide::types::ChatId;
use teloxide::types::UserId;
use teloxide::types::{Message, MessageId};

use self::message_helper::*;
pub mod message_helper;

pub struct MessageData {
    chat_id: ChatId,
    reply_message_id: Option<MessageId>,
    rep_giver_user_id: Option<UserId>,
    is_duplicate: bool,
}

impl MessageData {
    pub fn get_chat_id(&self) -> ChatId {
        self.chat_id
    }

    pub fn get_reply_message_id(&self) -> Option<MessageId> {
        self.reply_message_id
    }

    pub fn get_rep_giver_user_id(&self) -> Option<UserId> {
        self.rep_giver_user_id
    }

    pub fn get_data(message: &Message) -> MessageData {
        let mut data = MessageData {
            chat_id: message.chat.id,
            reply_message_id: get_replied_message_id(message),
            rep_giver_user_id: get_message_user_id(message),
            is_duplicate: false,
        };
        data.is_duplicate = is_duplicate_reputation(&data);

        data
    }
}
