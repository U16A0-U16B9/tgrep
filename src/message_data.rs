use teloxide::types::Message;
use teloxide::types::ChatId;
use teloxide::types::UserId;
use crate::services::config::triggers::TriggerType;

use self::services::*;
pub mod triggers;
pub mod services;

pub struct MessageData {
    valid: bool,
    chat_id: ChatId,
    rep_reciv_user_id: Option<UserId>,
    rep_giver_user_id: Option<UserId>,
    rep_reciv_user_name: Option<String>,
    rep_giver_user_name: Option<String>,
    is_rep_reciv_bot: bool,
    is_trigger: bool,
    trigger_type: TriggerType
}

impl MessageData {
    pub fn get_valid(& self) -> bool {
        self.valid
    }

    pub fn set_valid(&mut self, valid:bool) {
        self.valid = valid;
    }

    pub fn get_chat_id(& self) -> ChatId {
        self.chat_id
    }

    pub fn set_chat_id(&mut self, chat_id:ChatId) {
        self.chat_id = chat_id;
    }

    pub fn get_rep_reciv_user_id(& self) ->  Option<UserId> {
        self.rep_reciv_user_id
    }

    pub fn set_rep_reciv_user_id(&mut self, rep_reciv_user_id: Option<UserId>) {
        self.rep_reciv_user_id = rep_reciv_user_id;
    }

    pub fn get_rep_giver_user_id(& self) -> Option<UserId> {
        self.rep_giver_user_id
    }

    pub fn set_rep_giver_user_id(&mut self, rep_giver_user_id:Option<UserId>) {
        self.rep_giver_user_id = rep_giver_user_id;
    }

    pub fn get_rep_reciv_user_name(& self) -> &Option<String> {
        &self.rep_reciv_user_name
    }

    pub fn set_rep_reciv_user_name(&mut self, rep_reciv_user_name:Option<String>) {
        self.rep_reciv_user_name = rep_reciv_user_name;
    }

    pub fn get_rep_giver_user_name(& self) -> &Option<String> {
        &self.rep_giver_user_name
    }

    pub fn set_rep_giver_user_name(&mut self, rep_giver_user_name:Option<String>) {
        self.rep_giver_user_name = rep_giver_user_name;
    }

    pub fn get_is_rep_reciv_bot(& self) -> bool {
        self.is_rep_reciv_bot
    }

    pub fn set_is_rep_reciv_bot(&mut self, is_rep_reciv_bot:bool) {
        self.is_rep_reciv_bot = is_rep_reciv_bot;
    }

    pub fn get_is_trigger(& self) -> bool {
        self.is_trigger
    }

    pub fn set_is_trigger(&mut self, is_trigger: bool) {
        self.is_trigger = is_trigger;
    }


    pub fn get_trigger_type(& self) -> &TriggerType {
        &self.trigger_type
    }

    pub fn set_trigger_type(&mut self, trigger_type: TriggerType) {
        self.trigger_type = trigger_type;
    }

    pub fn new(chat_id:ChatId) -> MessageData {
        MessageData {
            valid: false,
            chat_id,
            rep_reciv_user_id: None,
            rep_giver_user_id: None,
            rep_reciv_user_name: None,
            rep_giver_user_name: None,
            is_rep_reciv_bot: false,
            is_trigger: false,
            trigger_type: TriggerType::None
        }
    }

    pub fn get_data(message: &Message) -> MessageData {
        let mut data = MessageData::new(message.chat.id);

        data.set_rep_reciv_user_id(get_replied_user_id(message));
        data.set_rep_giver_user_id(get_message_user_id(message));

        data.set_rep_reciv_user_name(get_replied_user_name(message));
        data.set_rep_giver_user_name(get_message_user_name(message));

        data.set_is_rep_reciv_bot(get_is_replied_user_bot(message));

        let trigger_type = get_message_trigger_type(message);
        data.set_is_trigger(get_is_message_trigger(&trigger_type));
        data.set_trigger_type(trigger_type);

        data.set_valid(calculate_if_data_is_valid(&data));

        data
    }
}
