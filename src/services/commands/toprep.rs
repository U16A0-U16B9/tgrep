use std::cmp;
use teloxide::prelude::{Message, UserId};
use teloxide::types::User;
use crate::services::commands;
use crate::services::commands::Command;
use crate::services::config::settings::Settings;
use crate::services::data::reputations::Reputations;
use crate::services::data::user_list::UserList;

const TOP_REP_COMMAND_TRIGGER: &str = "/toprep";
const DEFAULT_TOP_REP_LIMIT: usize = 10;

pub struct TopRep {}

impl Command for TopRep {
    fn is_valid_command(&self, message: &Message) -> bool {
        commands::default_command_validation(message)
        && (
            commands::is_command_match(message, TOP_REP_COMMAND_TRIGGER)
            || commands::is_command_match_with_param(message, TOP_REP_COMMAND_TRIGGER)
        )
    }

    fn execute(&self, _message: &Message) {
    }

    fn response(&self, message: &Message) -> String {
        let reputations = Reputations::load();
        // let chat = message.chat;
        let chat_reputations = reputations.chats.get(&message.chat.id);
        return match chat_reputations {
            None => {
                "No reputations in this chat".to_string()
            }
            Some(_reputations) => {
                let users = UserList::load();
                let mut response = "".to_string();
                let mut sorted: Vec<(&UserId, &i64)> = _reputations.iter().collect();
                sorted.sort_by(|a, b| b.1.cmp(&a.1));
                let limit = cmp::min(sorted.len(),limit_results(message));
                sorted = sorted[..limit].to_owned();

                for (user, rep) in sorted {
                     match users.user_list.get(user) {
                        None => {
                            response.push_str(format!("{} - {}\n",  user.0, rep).as_str())
                        }
                        Some(_user) => {
                            response.push_str(format!("{} - {}\n",  generate_display_name(_user), rep).as_str())
                        }
                    }
                }

                response
            }
        }
    }

    fn description(&self) -> String {
        commands::parse_description(TOP_REP_COMMAND_TRIGGER, "Displays top reputations")
    }
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

fn limit_results(message: &Message) -> usize {
    if commands::is_command_match(message, TOP_REP_COMMAND_TRIGGER) {
        return DEFAULT_TOP_REP_LIMIT;
    }
    return match message.text() {
        None => { DEFAULT_TOP_REP_LIMIT }
        Some(message_text) => {
            let words: Vec<&str> = message_text.split_whitespace().collect();
            let param = words.get(1);
            match param {
                None => {
                    DEFAULT_TOP_REP_LIMIT
                }
                Some(_param) => {
                    let limit: usize = _param.to_string().parse().unwrap_or(10);
                    limit
                }
            }
        }
    }
}