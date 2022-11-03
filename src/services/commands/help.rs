use crate::services::commands;
use crate::services::commands::Command;
use teloxide::prelude::Message;

const HELP_COMMAND_TRIGGER: &str = "/help";

pub struct Help {}
impl Command for Help {
    fn is_valid_command(&self, message: &Message, bot_username: &Option<String>) -> bool {
        commands::default_command_validation(message) && commands::is_command_match(message, HELP_COMMAND_TRIGGER, bot_username)
    }

    fn execute(&self, _message: &Message) {}

    fn response(&self, _message: &Message, _: &Option<String>) -> String {
        let mut response = "".to_string();
        let command_list = match _message.chat.is_private() {
            true => commands::get_command_private_list(),
            false => commands::get_command_group_list()
        };
        for command in command_list {
            response.push_str(format!("{}", command.description()).as_str());
        }
        response
    }

    fn description(&self) -> String {
        commands::parse_description(HELP_COMMAND_TRIGGER, "Displays this message")
    }

    fn whoami(&self) -> &str {
        HELP_COMMAND_TRIGGER
    }
}
