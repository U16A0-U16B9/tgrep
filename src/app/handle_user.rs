use crate::objects::user::generate_display_name;
use crate::services::data::user_list::UserList;
use log::{info, warn};
use teloxide::types::Message;

pub fn save_user(message: &Message) {
    let mut users = UserList::load();
    let message_user = message.from();

    match message_user {
        None => {
            warn!("Cannot save user");
        }
        Some(user) => match users.user_list.get(&user.id) {
            None => {
                users.user_list.insert(user.id, user.clone());
                UserList::save(users);
                info!("user {} saved", generate_display_name(user))
            }
            Some(saved_user) => {
                if generate_display_name(saved_user) != generate_display_name(user) {
                    users.user_list.insert(user.id, user.clone());
                    UserList::save(users);
                    info!("user {} updated", generate_display_name(user))
                }
            }
        },
    }
}
