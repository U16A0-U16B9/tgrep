use teloxide::types::{Message};
use teloxide::types::User;
use teloxide::types::UserId;
use crate::services::config::settings::Settings;
use crate::services::config::triggers::{Triggers, TriggerType};
use super::MessageData;

pub fn get_replied_user_id(message: &Message) -> Option<UserId> {
    match message.reply_to_message() {
        Some(_replied) => {
            match _replied.from() {
                Some(_user) => {
                    Some(_user.id)
                },
                None => None,
            }
        }
        None => {
            None
        },
    }
}

pub fn get_message_user_id(message: &Message) -> Option<UserId> {
    match message.from() {
        Some(_sender) => {
            Some(_sender.id)
        },
        None => {
            None
        },
    }
}

pub fn get_replied_user_name(message: &Message) -> Option<String> {
    match message.reply_to_message() {
        Some(_replied) => {
            match _replied.from() {
                Some(_user) => {
                    Some(generate_display_name(_user))
                },
                None => None,
            }
        }
        None => {
            None
        },
    }
}

pub fn get_message_user_name(message: &Message) -> Option<String> {
    match message.from() {
        Some(_sender) => {
            Some(generate_display_name(_sender))
        },
        None => {
            None
        },
    }
}

pub fn get_is_replied_user_bot(message: &Message) -> bool {
    match message.reply_to_message() {
        Some(_replied) => {
            match _replied.from() {
                Some(_user) => {
                    _user.is_bot
                },
                None => false,
            }
        }
        None => {
            false
        },
    }
}

pub fn get_message_trigger_type(message: &Message) -> TriggerType {
    match message.text() {
        Some(_text) => {
            get_trigger_type(_text.to_string(), false)
        },
        None => {
            match message.sticker() {
                Some(_sticker) => {
                    get_trigger_type(_sticker.clone().file_unique_id, true)
                }
                None => {
                    return TriggerType::None;
                }
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

pub fn calculate_if_data_is_valid(data: &MessageData) -> bool {
    if !data.get_is_trigger() {
        return false;
    }

    let settings = Settings::load();

    if !settings.can_rep_bot && data.get_is_rep_reciv_bot() {
        return false;
    }

    let rep_giver_user_id;
    let rep_reciv_user_id;

    match data.get_rep_giver_user_id() {
        Some(_user_id) => (rep_giver_user_id = _user_id.clone()),
        None => return false,
    }

    match data.get_rep_reciv_user_id() {
        Some(_user_id) => (rep_reciv_user_id = _user_id.clone()),
        None => return false,
    }

    if !settings.can_self_rep && rep_giver_user_id == rep_reciv_user_id {
        return false;
    }

    true
}

fn generate_display_name(user: &User) -> String {
    let settings = Settings::load();
    let fullname = user.full_name();

    if !settings.display_username {
        fullname
    }
    else {
        let username = &user.username;
        match username {
            Some(_username) =>  {
                _username.to_string()
            },
            None => {
                fullname
            }
        }
    }
}

fn get_trigger_type(string: String, strict: bool) -> TriggerType {
    let triggers = Triggers::load();
    for trigger in triggers.positive.iter() {
        if strict && string.contains(trigger)
            || string.to_lowercase().contains(trigger) {
            return TriggerType::Positive;
        }
    }

    for trigger in triggers.negative.iter() {
        if strict && string.contains(trigger)
            || string.to_lowercase().contains(trigger) {
            return TriggerType::Negative;
        }
    }

    TriggerType::None
}