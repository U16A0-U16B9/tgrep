use teloxide::types::{Message};
use crate::services::commands::help::Help;
use crate::services::commands::toprep::TopRep;

pub mod help;
pub mod toprep;

pub fn get_command_list() -> Vec<&'static dyn Command> {
    vec![
        &Help {},
        &TopRep {}
    ]
}

pub trait Command {
    fn is_valid_command(&self, message: &Message) -> bool;
    fn execute(&self, message: &Message);
    fn response(&self, message: &Message) -> String;
    fn description(&self) -> String;
}

pub fn default_command_validation(message: &Message) -> bool {
    match message.reply_to_message() {
        Some(_) => { return false }
        _ => { }
    }

    match message.from() {
        None => { return false }
        Some(_user) => {
            if _user.is_bot {
                return false
            }
        }
    }

    true
}

pub fn parse_description(command: &str, description: &str) -> String {
    format!("{} - {}\n",  command, description)
}

pub fn is_command_match(message: &Message, command: &str) -> bool {
    let mut valid= true;
    match  message.text() {
        None => { valid = false; }
        Some(_text) => {
            if _text != command {
                valid = false;
            }
        }
    }
    valid
}

pub fn is_command_match_with_param(message: &Message, command: &str) -> bool {
    let mut valid= true;
    match  message.text() {
        None => { valid = false; }
        Some(_text) => {
            if !_text.to_string().starts_with((command.to_string() + " ").as_str()) {
                valid = false;
            }
        }
    }
    valid
}