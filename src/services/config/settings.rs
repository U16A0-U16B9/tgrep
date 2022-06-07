use serde::{Serialize, Deserialize};
use crate::services::persistence_manager::file_manager::FileManager;
use crate::services::persistence_manager::{ConfigType, PersistenceManager};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub bot_id: Option<String>,
    pub can_self_rep: bool,
    pub can_rep_bot: bool,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            bot_id: None,
            can_self_rep: false,
            can_rep_bot: false
        }
    }

    pub fn load() -> Settings {
        let settings_text = FileManager::load_config(ConfigType::Settings);
        match settings_text {
            None => { Self::save(Self::new()) }
            Some(_settings_text) => {
                let settings_result: Result<Settings, serde_json::Error> = serde_json::from_str(_settings_text.as_str());
                match settings_result {
                    Ok(_settings) => {  _settings }
                    _ => {  Self::save(Self::new()) }
                }
            }
        }
    }

    fn save(settings: Settings) -> Settings {
        let settings_text = serde_json::to_string(&settings);
        match settings_text {
            Ok(_settings_text) => {
                FileManager::save_config(ConfigType::Settings, _settings_text);
            }
            Err(_a) => { panic!("{}", _a.to_string()) }
        }
        settings
    }
}

#[cfg(test)]
mod settings_tests {
    use super::*;

    #[test]
    fn test_new() {
        let settings = Settings::new();
        assert_eq!(settings.bot_id, None);
        assert_eq!(settings.can_self_rep, false);
        assert_eq!(settings.can_rep_bot, false);
    }

    #[test]
    fn test_load_save() {
        let mut settings = Settings::load();
        let can_rep_bot = settings.can_rep_bot;

        settings.can_rep_bot = !settings.can_rep_bot;
        Settings::save(settings);

        let mut settings = Settings::load();
        assert_ne!(settings.can_rep_bot, can_rep_bot);

        // cleanup
        settings.can_rep_bot = !settings.can_rep_bot;
        Settings::save(settings);
    }
}