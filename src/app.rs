use std::future::Future;
use teloxide::prelude::*;
use handle_rep::HandledReputation;
use message_data::MessageData;

pub mod message_data;
pub mod handle_rep;

pub fn init() -> impl Future {
    pretty_env_logger::init();
    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {

        let data = MessageData::get_data(&message);
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
