use crate::services::commands;
use log::{error, info};
use teloxide::{Bot};
use teloxide::prelude::{AutoSend, Requester};
use teloxide::types::{Message};

pub fn execute(message: &Message, bot_username: Option<String>) -> (bool, Option<String>) {
    for command in commands::get_command_list() {
        if command.is_valid_command(message, &bot_username) {
            command.execute(message);
            info!("{} command executed", command.whoami());
            return (true, Some(command.response(message, &bot_username)));
        }
    }
    (false, None)
}

pub async fn get_bot_username(bot: &AutoSend<Bot>) -> Option<String> {
    let me_result = bot.get_me().await;
    match me_result {
        Ok(me) => {
            me.user.username
        }
        Err(err) => {
            error!("cannot get bot: {}", err.to_string());
            None
        }
    }
}
