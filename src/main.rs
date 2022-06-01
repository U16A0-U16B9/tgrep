use message_data::MessageData;
use handle_rep::*;
use teloxide::prelude::*;
pub mod message_data;
pub mod handle_rep;

#[tokio::main]
async fn main() {
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
                        _handled_reputation.giver_username.unwrap(),
                        _handled_reputation.operation,
                        _handled_reputation.reciv_username.unwrap(),
                        _handled_reputation.reciv_reputation
                    )
                ).await?;
            },
            None => (),
        }

        respond(())
    })
        .await;
}
