use super::MessageData;
use crate::app::reputation_message::ReputationMessage;
use crate::objects::user;
use crate::services::config::settings::Settings;
use crate::services::config::triggers::{ChatTrigger, TriggerType, Triggers, TRIGGER_VERSION};
use crate::services::data::reputation_history::ReputationHistory;
use std::ops::Deref;
use teloxide::types::{ChatId, User, UserId};
use teloxide::types::{Message, MessageId};

pub fn get_replied_user(message: &Message) -> Option<User> {
    match message.reply_to_message() {
        Some(_replied) => match _replied.from() {
            Some(_user) => Some(_user.clone()),
            None => None,
        },
        None => None,
    }
}

pub fn get_replied_message(message: &Message) -> Option<Message> {
    match message.reply_to_message() {
        Some(replied) => Some(message.clone()),
        None => None,
    }
}

pub fn get_message_user(message: &Message) -> Option<User> {
    match message.from() {
        Some(_sender) => Some(_sender.clone()),
        None => None,
    }
}

pub fn get_chat_triggers(message: &Message) -> Vec<ChatTrigger> {
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

pub fn is_duplicate(message: &Message) -> bool {
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
                if !chat_trigger.is_sticker {
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

pub fn get_replied_user_id(message: &Message) -> Option<UserId> {
    match message.reply_to_message() {
        Some(_replied) => match _replied.from() {
            Some(_user) => Some(_user.id),
            None => None,
        },
        None => None,
    }
}

pub fn get_message_user_id(message: &Message) -> Option<UserId> {
    match message.from() {
        Some(_sender) => Some(_sender.id),
        None => None,
    }
}

pub fn get_replied_user_name(message: &Message) -> Option<String> {
    match message.reply_to_message() {
        Some(_replied) => match _replied.from() {
            Some(_user) => Some(user::generate_display_name(_user)),
            None => None,
        },
        None => None,
    }
}

pub fn get_replied_message_id(message: &Message) -> Option<MessageId> {
    match message.reply_to_message() {
        Some(replied) => Some(MessageId { message_id: replied.id }),
        None => None,
    }
}

pub fn get_message_user_name(message: &Message) -> Option<String> {
    match message.from() {
        Some(_sender) => Some(user::generate_display_name(_sender)),
        None => None,
    }
}

pub fn get_is_replied_user_bot(message: &Message) -> bool {
    match message.reply_to_message() {
        Some(_replied) => match _replied.from() {
            Some(_user) => _user.is_bot,
            None => false,
        },
        None => false,
    }
}

pub fn get_message_trigger_type(message: &Message) -> TriggerType {
    match message.text() {
        Some(_text) => get_trigger_type(_text.to_string(), false),
        None => match message.sticker() {
            Some(_sticker) => get_trigger_type(_sticker.clone().file_unique_id, true),
            None => {
                return TriggerType::None;
            }
        },
    }
}

pub fn get_is_message_trigger(trigger_type: &TriggerType) -> bool {
    match trigger_type {
        TriggerType::None => false,
        TriggerType::Positive => true,
        TriggerType::Negative => true,
    }
}

pub fn calculate_if_data_is_valid(reputation_message: &ReputationMessage) -> bool {
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

pub fn is_duplicate_reputation(data: &MessageData) -> bool {
    let reputation_history = ReputationHistory::load();
    return match reputation_history.chats.get(&data.get_chat_id()) {
        None => false,
        Some(chat_reputation_history) => {
            for reputation_history_item in chat_reputation_history {
                if reputation_history_item.sender == data.get_rep_giver_user_id()
                    && reputation_history_item.reply_message_id == data.get_reply_message_id()
                {
                    return true;
                }
            }
            false
        }
    };
}

fn get_trigger_type(string: String, strict: bool) -> TriggerType {
    let triggers = Triggers::load();
    // for trigger in triggers.positive.iter() {
    //     if strict && string.contains(trigger) || string.to_lowercase().contains(trigger) {
    //         return TriggerType::Positive;
    //     }
    // }
    //
    // for trigger in triggers.negative.iter() {
    //     if strict && string.contains(trigger) || string.to_lowercase().contains(trigger) {
    //         return TriggerType::Negative;
    //     }
    // }

    TriggerType::None
}
