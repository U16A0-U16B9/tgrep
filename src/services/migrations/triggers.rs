use crate::services::config::triggers::Triggers;
use crate::services::config::triggers::TRIGGER_VERSION;
use crate::services::migrations::triggers::trigger_v0::TriggersV0;
use crate::services::migrations::triggers::trigger_v1::TriggersV1;
use crate::services::migrations::Migrations;
use log::info;
use serde::Serialize;
use serde_json::Error;

mod trigger_v0;
mod trigger_v1;

pub struct TriggerMigration {}

impl TriggerMigration {
    fn convert_to_trigger<T: Serialize>(migration_trigger: T) -> Result<Triggers, Error> {
        let result = serde_json::to_string(&migration_trigger);
        match result {
            Ok(string) => {
                let trigger: Result<Triggers, serde_json::Error> = serde_json::from_str(string.as_str());
                trigger
            }
            Err(e) => Err(e),
        }
    }

    pub fn migrate_version_v1(triggers_option: Option<TriggersV0>) -> Result<Triggers, Option<TriggersV1>> {
        match triggers_option {
            None => {
                panic!("❌ Trigger migration failed: Unable to load triggers")
            }
            Some(triggers_v0) => {
                let (triggers_v1, triggers_option) = TriggersV1::migrate(triggers_v0);

                match triggers_option {
                    None => Err(triggers_v1),
                    Some(triggers) => Ok(Triggers::save(triggers)),
                }
            }
        }
    }
}

impl Migrations for TriggerMigration {
    fn get_current_version() -> usize {
        match TriggersV0::load() {
            None => {}
            Some(_) => return 0,
        }
        match TriggersV1::load() {
            None => {}
            Some(_) => return 1,
        }

        usize::MAX
    }

    fn get_latest_version() -> usize {
        return TRIGGER_VERSION;
    }

    fn migrate() {
        info!("🚀 Migrating Triggers");
        if Self::get_current_version() == usize::MAX {
            panic!("❌ Trigger migration failed: Unknown current version")
        }
        if Self::get_current_version() == Self::get_latest_version() {
            return info!("🚀 Trigger migration already up to date");
        }
        if Self::get_current_version() > Self::get_latest_version() {
            panic!("❌ Trigger migration failed: Outdated bot")
        }

        let mut current_version = Self::get_current_version();
        while current_version < Self::get_latest_version() {
            match current_version {
                0 => {
                    let triggers_option = TriggersV0::load();
                    let migration = Self::migrate_version_v1(triggers_option);
                    match migration {
                        Ok(_) => {
                            info!("🚀 Trigger migration completed");
                            break;
                        }
                        Err(_) => {
                            current_version = current_version + 1;
                        }
                    }
                }
                _ => {
                    panic!("❌ Trigger migration failed: migration not implemented")
                }
            }
        }
    }
}
