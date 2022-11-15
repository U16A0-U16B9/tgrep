use crate::services::config::settings::Settings;
use crate::services::config::triggers::{ChatTrigger, Triggers};
use crate::services::data::reputation_history::ReputationHistory;
use teloxide::prelude::UserId;
use teloxide::types::{ChatId, Message, MessageId, User};

pub struct ReputationMessage {
    pub chat_id: ChatId,
    pub message_id: MessageId,
    pub rep_reciv: Option<User>,
    pub rep_giver: Option<User>,
    pub chat_triggers: Vec<ChatTrigger>,
    pub reply_message: Option<Message>,
    pub is_duplicate: bool,
    pub is_valid: bool,
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
            is_valid: false,
        };

        reputation_message.is_valid = calculate_if_data_is_valid(&reputation_message);
        reputation_message
    }
}

fn get_replied_user(message: &Message) -> Option<User> {
    match message.reply_to_message() {
        Some(_replied) => match _replied.from() {
            Some(_user) => Some(_user.clone()),
            None => None,
        },
        None => None,
    }
}

fn calculate_if_data_is_valid(reputation_message: &ReputationMessage) -> bool {
    if !reputation_message.chat_triggers.iter().count() == 0 {
        return false;
    }

    let settings = Settings::load();

    return match reputation_message.rep_reciv.as_ref() {
        None => false,
        Some(rep_reciv_user) => {
            if !settings.can_rep_bot && rep_reciv_user.is_bot {
                return false;
            }
            let rep_giver_user_id = reputation_message.rep_giver.as_ref().map(|user| user.id).unwrap();

            if !settings.can_self_rep && rep_giver_user_id == rep_reciv_user.id {
                return false;
            }

            if settings.disable_multiple_reps && reputation_message.is_duplicate {
                return false;
            }

            true
        }
    };
}

fn get_chat_triggers(message: &Message) -> Vec<ChatTrigger> {
    match message.text() {
        Some(text) => get_message_trigger(message.chat.id, text.to_string(), false),
        None => match message.sticker() {
            Some(sticker) => get_message_trigger(message.chat.id, sticker.clone().file_unique_id, true),
            _ => {
                vec![]
            }
        },
    }
}

fn get_message_user(message: &Message) -> Option<User> {
    match message.from() {
        Some(_sender) => Some(_sender.clone()),
        None => None,
    }
}

fn is_duplicate(message: &Message) -> bool {
    let reputation_history = ReputationHistory::load();
    return match reputation_history.chats.get(&message.chat.id) {
        None => false,
        Some(chat_reputation_history) => {
            for reputation_history_item in chat_reputation_history {
                let user_id: Option<UserId> = message.from().cloned().map(|user| user.id);
                let reply_message_id = message.reply_to_message().map(|rm| MessageId { message_id: rm.id });
                if reputation_history_item.sender == user_id
                    && reputation_history_item.reply_message_id == reply_message_id
                {
                    return true;
                }
            }
            false
        }
    };
}

fn get_message_trigger(chat_id: ChatId, string: String, is_sticker: bool) -> Vec<ChatTrigger> {
    let triggers = Triggers::load();
    let mut message_chat_triggers: Vec<ChatTrigger> = vec![];
    let chat_triggers_options = triggers.chat.get(&chat_id).cloned();
    match chat_triggers_options {
        None => {}
        Some(chat_triggers) => {
            for chat_trigger in chat_triggers.iter().cloned() {
                let mut trigger = chat_trigger.clone().trigger;
                if !is_sticker {
                    trigger = trigger.to_lowercase()
                }

                if chat_trigger.is_wildcard && string.contains(&trigger) {
                    message_chat_triggers.push(chat_trigger.clone())
                } else if !chat_trigger.is_wildcard && string.eq(&trigger) {
                    message_chat_triggers.push(chat_trigger)
                }
            }
        }
    };
    message_chat_triggers
}
