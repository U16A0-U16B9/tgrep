use std::future::Future;
use teloxide::prelude::*;
use handle_rep::HandledReputation;
use message_data::MessageData;
use super::services::enviroment_variables;

pub mod message_data;
pub mod handle_rep;
pub mod handle_cmd;

pub fn init() -> impl Future {
    pretty_env_logger::init();
    enviroment_variables::load();

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {

        let data = MessageData::get_data(&message);
        let (is_command, command_message) = handle_cmd::execute(&message);
        if is_command {
            let command_message = command_message
                .unwrap_or("Unknown command error".to_string());

            bot.send_message(
                data.get_chat_id(),
                format!("{} ", command_message)
            ).await?;

            return respond(());
        }

        let result = HandledReputation::handle_rep(&data);
        match result {
            Some(_handled_reputation) => {
                bot.send_message(
                    data.get_chat_id(),
                    format!(
                        "{} has {} reputation of {} to {}",
                        _handled_reputation.giver_username,
                        _handled_reputation.operation,
                        _handled_reputation.reciv_username,
                        _handled_reputation.reciv_reputation
                    )
                ).await?;
            },
            None => (),
        }

        respond(())
    })
}
