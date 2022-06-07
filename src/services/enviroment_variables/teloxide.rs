use std::env;
use crate::services::config::settings::Settings;

const TELOXIDE_ENV_KEY: &str = "TELOXIDE_TOKEN";

pub fn load_teloxide_env_variable() {
    let settings = Settings::load();

    match settings.bot_id {
        None => {}
        Some(_bot_id) => {
            env::set_var(TELOXIDE_ENV_KEY, _bot_id);
        }
    }
}