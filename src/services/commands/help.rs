use crate::services::commands;
use crate::services::commands::Command;
use teloxide::prelude::Message;

const HELP_COMMAND_TRIGGER: &str = "/help";

pub struct Help {}
impl Command for Help {
    fn is_valid_command(&self, message: &Message) -> bool {
        commands::default_command_validation(message) && commands::is_command_match(message, HELP_COMMAND_TRIGGER)
    }

    fn execute(&self, _message: &Message) {}

    fn response(&self, _message: &Message) -> String {
        let mut response = "".to_string();
        for command in commands::get_command_list() {
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
