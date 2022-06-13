use crate::services::persistence_manager::file_manager::FileManager;
use crate::services::persistence_manager::{ConfigType, PersistenceManager};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub bot_id: Option<String>,
    pub can_self_rep: bool,
    pub can_rep_bot: bool,
    pub display_username: bool,
    pub save_history: bool,
    pub disable_multiple_reps: bool,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            bot_id: None,
            can_self_rep: false,
            can_rep_bot: false,
            display_username: false,
            save_history: true,
            disable_multiple_reps: true,
        }
    }

    pub fn load() -> Settings {
        let settings_text = FileManager::load_config(ConfigType::Settings);
        match settings_text {
            None => Self::save(Self::new()),
            Some(_settings_text) => {
                let settings_result: Result<Settings, serde_json::Error> =
                    serde_json::from_str(_settings_text.as_str());
                match settings_result {
                    Ok(_settings) => _settings,
                    _ => Self::save(Self::new()),
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
            Err(_a) => {
                error!("Cannot save {}", ConfigType::Settings.to_string());
                panic!("{}", _a.to_string())
            }
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
        assert_eq!(settings.display_username, false);
        assert_eq!(settings.save_history, true);
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
