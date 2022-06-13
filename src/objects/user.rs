use crate::services::config::settings::Settings;
use teloxide::types::User;

pub fn generate_display_name(user: &User) -> String {
    let settings = Settings::load();
    let fullname = user.full_name();

    if !settings.display_username {
        fullname
    } else {
        let username = &user.username;
        match username {
            Some(_username) => _username.to_string(),
            None => fullname,
        }
    }
}
