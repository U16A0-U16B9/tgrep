use crate::app::reputation_message::ReputationMessage;
use crate::objects::messages::{MessageSender, ParseMessage};
use crate::services::bootstrap;
use handle_rep::HandledReputation;
use message_data::MessageData;
use std::future::Future;
use teloxide::prelude::*;

pub mod handle_cmd;
pub mod handle_rep;
pub mod handle_user;
pub mod message_data;
pub mod reputation_message;

pub fn init() -> impl Future {
    bootstrap::start();
    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        handle_user::save_user(&message);
        let data = MessageData::get_data(&message);
        let reputation = ReputationMessage::new(&message);
        let (is_command, command_message) = handle_cmd::execute(&message);
        if is_command {
            let command_message = command_message.unwrap_or("Unknown command error".to_string());

            MessageSender::new(data.get_chat_id(), command_message).send(bot).await;
            return respond(());
        }

        let result = HandledReputation::handle_rep(&reputation);
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
