use super::services::environment_variables;
use crate::objects::messages::{MessageSender, ParseMessage};
use handle_rep::HandledReputation;
use message_data::MessageData;
use std::future::Future;
use teloxide::prelude::*;
use crate::services::bootstrap;

pub mod handle_cmd;
pub mod handle_rep;
pub mod handle_user;
pub mod message_data;

pub fn init() -> impl Future {
    pretty_env_logger::init();
    bootstrap::start();
    environment_variables::load();

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        handle_user::save_user(&message);
        let data = MessageData::get_data(&message);

        let (is_command, command_message) = handle_cmd::execute(&message);
        if is_command {
            let command_message = command_message.unwrap_or("Unknown command error".to_string());

            MessageSender::new(data.get_chat_id(), command_message).send(bot).await;
            return respond(());
        }

        let result = HandledReputation::handle_rep(&data);
        match result {
            Some(_handled_reputation) => {
                MessageSender::new(data.get_chat_id(), _handled_reputation.parse())
                    .send(bot)
                    .await;
            }
            None => (),
        }

        respond(())
    })
}
