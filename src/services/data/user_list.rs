use crate::services::data::Data;
use crate::services::persistence_manager::file_manager::FileManager;
use crate::services::persistence_manager::{DataType, PersistenceManager};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use teloxide::prelude::UserId;
use teloxide::types::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserList {
    pub user_list: HashMap<UserId, User>,
}

impl UserList {
    pub fn new() -> UserList {
        UserList {
            user_list: HashMap::new(),
        }
    }

    pub fn load() -> UserList {
        let user_list_text = FileManager::load_data(DataType::UserList);
        return match user_list_text {
            Some(_text) => {
                let v: Result<UserList, serde_json::Error> = serde_json::from_str(_text.as_str());
                match v {
                    Ok(_reputations) => _reputations,
                    Err(_) => UserList::new(),
                }
            }
            None => UserList::new(),
        };
    }

    pub fn save(reputations: UserList) -> UserList {
        let user_list_text = serde_json::to_string(&reputations);
        match user_list_text {
            Ok(_user_list_text) => FileManager::save_data(DataType::UserList, _user_list_text),
            Err(_a) => panic!("{}", _a.to_string()),
        }
        reputations
    }
}

impl Data for UserList {}

#[cfg(test)]
mod reputation_tests {
    use super::*;

    #[test]
    fn test_new() {
        let users = UserList::new();
        assert_eq!(users.user_list.len(), 0)
    }

    #[test]
    fn test_load_save() {
        let user_id = UserId(17);
        let user = User {
            id: user_id,
            is_bot: false,
            first_name: "test".to_string(),
            last_name: None,
            username: None,
            language_code: None,
        };

        let mut users = UserList::load();
        users.user_list.insert(user_id, user);

        UserList::save(users);
        let mut users = UserList::load();

        assert!(users.user_list.contains_key(&user_id));
        assert_eq!(users.user_list[&user_id].first_name, "test".to_string());

        //cleanup
        users.user_list.remove(&user_id);
    }
}
