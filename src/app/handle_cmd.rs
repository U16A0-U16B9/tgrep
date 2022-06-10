use crate::services::commands;
use teloxide::types::Message;

pub fn execute(message: &Message) -> (bool, Option<String>) {
    for command in commands::get_command_list() {
        if command.is_valid_command(message) {
            command.execute(message);
            return (true, Some(command.response(message)));
        }
    }
    (false, None)
}
