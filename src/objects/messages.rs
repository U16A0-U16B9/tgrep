use log::{error, info};
use teloxide::prelude::AutoSend;
use teloxide::prelude::*;
use teloxide::types::ChatId;
use teloxide::Bot;

pub trait ParseMessage {
    fn parse(&self) -> String;
}

pub struct MessageSender {
    chat_id: ChatId,
    message_text: String,
}

impl MessageSender {
    pub fn new(chat_id: ChatId, message_text: String) -> MessageSender {
        MessageSender { chat_id, message_text }
    }

    pub async fn send(&self, bot: AutoSend<Bot>) {
        let result = bot.send_message(self.chat_id, format!("{} ", self.message_text)).await;
        match result {
            Ok(message) => {
                info!("message {} sent", message.id);
            }
            Err(_) => {
                error!("message couldn't send:")
            }
        }
    }
}
