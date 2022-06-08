use teloxide::types::{Message};
use crate::services::data::user_list::UserList;

pub fn save_user(message: &Message) {
    let mut users = UserList::load();
    let message_user = message.from();

    match message_user {
        None => {}
        Some(_user) => {
            match users.user_list.get(&_user.id) {
                None => {
                    users.user_list.insert(_user.id, _user.clone());
                    UserList::save(users);
                }
                Some(_) => {}
            }
        }
    }
}