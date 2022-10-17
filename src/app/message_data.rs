use crate::services::config::triggers::TriggerType;
use teloxide::types::ChatId;
use teloxide::types::UserId;
use teloxide::types::{Message, MessageId};

use self::message_helper::*;
pub mod message_helper;

pub struct MessageData {
    valid: bool,
    chat_id: ChatId,
    message_id: MessageId,
    reply_message_id: Option<MessageId>,
    rep_reciv_user_id: Option<UserId>,
    rep_giver_user_id: Option<UserId>,
    rep_reciv_user_name: Option<String>,
    rep_giver_user_name: Option<String>,
    is_rep_reciv_bot: bool,
    is_trigger: bool,
    trigger_type: TriggerType,
    is_duplicate: bool,
}

impl MessageData {
    pub fn get_valid(&self) -> bool {
        self.valid
    }

    pub fn get_chat_id(&self) -> ChatId {
        self.chat_id
    }

    pub fn get_message_id(&self) -> MessageId {
        self.message_id
    }

    pub fn get_reply_message_id(&self) -> Option<MessageId> {
        self.reply_message_id
    }

    pub fn get_rep_reciv_user_id(&self) -> Option<UserId> {
        self.rep_reciv_user_id
    }

    pub fn get_rep_giver_user_id(&self) -> Option<UserId> {
        self.rep_giver_user_id
    }

    pub fn get_rep_reciv_user_name(&self) -> &Option<String> {
        &self.rep_reciv_user_name
    }

    pub fn get_rep_giver_user_name(&self) -> &Option<String> {
        &self.rep_giver_user_name
    }

    pub fn get_is_rep_reciv_bot(&self) -> bool {
        self.is_rep_reciv_bot
    }

    pub fn get_is_trigger(&self) -> bool {
        self.is_trigger
    }

    pub fn get_is_duplicate(&self) -> bool {
        self.is_duplicate
    }

    pub fn get_trigger_type(&self) -> &TriggerType {
        &self.trigger_type
    }

    pub fn get_data(message: &Message) -> MessageData {
        let trigger_type = get_message_trigger_type(message);
        let mut data = MessageData {
            valid: false,
            chat_id: message.chat.id,
            message_id: MessageId { message_id: message.id },
            reply_message_id: get_replied_message_id(message),
            rep_reciv_user_id: get_replied_user_id(message),
            rep_giver_user_id: get_message_user_id(message),
            rep_reciv_user_name: get_replied_user_name(message),
            rep_giver_user_name: get_message_user_name(message),
            is_rep_reciv_bot: get_is_replied_user_bot(message),
            is_trigger: get_is_message_trigger(&trigger_type),
            is_duplicate: false,
            trigger_type,
        };
        data.is_duplicate = is_duplicate_reputation(&data);
        //data.valid = calculate_if_data_is_valid(&data);

        data
    }
}
