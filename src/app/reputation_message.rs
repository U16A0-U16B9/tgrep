use teloxide::types::{ChatId, Message, MessageId, User};
use crate::app::message_data::message_helper::{calculate_if_data_is_valid, get_chat_triggers, get_message_user, get_replied_message, get_replied_user, is_duplicate};
use crate::services::config::triggers::ChatTrigger;

pub struct ReputationMessage {
    pub chat_id: ChatId,
    pub message_id: MessageId,
    pub rep_reciv: Option<User>,
    pub rep_giver: Option<User>,
    pub chat_triggers: Vec<ChatTrigger>,
    pub reply_message: Option<Message>,
    pub is_duplicate: bool,
    pub is_valid: bool
}

impl ReputationMessage {
    pub fn new(message: &Message) -> ReputationMessage {
        let mut reputation_message = ReputationMessage {
            chat_id: message.chat.id,
            message_id: MessageId { message_id: message.id },
            rep_reciv: get_replied_user(message),
            rep_giver: get_message_user(message),
            chat_triggers: get_chat_triggers(message),
            reply_message: message.reply_to_message().cloned(),
            is_duplicate: is_duplicate(message),
            is_valid: false
        };

        reputation_message.is_valid = calculate_if_data_is_valid(&reputation_message);
        reputation_message
    }
}