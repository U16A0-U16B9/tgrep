use crate::services::config::settings::Settings;
use log::{info, warn};
use std::env;

const TELOXIDE_ENV_KEY: &str = "TELOXIDE_TOKEN";

pub fn load_teloxide_env_variable() {
    let settings = Settings::load();

    match settings.bot_id {
        None => {
            warn!("{} environment variable missing", TELOXIDE_ENV_KEY)
        }
        Some(_bot_id) => {
            env::set_var(TELOXIDE_ENV_KEY, _bot_id);
            info!("{} environment variable set", TELOXIDE_ENV_KEY)
        }
    }
}
