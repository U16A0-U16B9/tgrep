use crate::app::reputation_message::ReputationMessage;
use crate::objects::messages::{MessageSender, ParseMessage};
use crate::services::bootstrap;
use handle_rep::HandledReputation;
use std::future::Future;
use teloxide::prelude::*;

pub mod handle_cmd;
pub mod handle_rep;
pub mod handle_user;
pub mod reputation_message;

pub fn init() -> impl Future {
    bootstrap::start();
    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        handle_user::save_user(&message);
        let bot_username = handle_cmd::get_bot_username(&bot).await;

        let (is_command, command_message) = handle_cmd::execute(&message, bot_username);
        if is_command {
            let command_message = command_message.unwrap_or("Unknown command error".to_string());

            MessageSender::new(message.chat.id, command_message).send(bot).await;
            return respond(());
        }

        let reputation = ReputationMessage::new(&message);
        let result = HandledReputation::handle_rep(&reputation);
        match result {
            Some(_handled_reputation) => {
                MessageSender::new(message.chat.id, _handled_reputation.parse())
                    .send(bot)
                    .await;
            }
            None => (),
        }

        respond(())
    })
}
