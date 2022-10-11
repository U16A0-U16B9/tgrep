use log::{info};
use crate::services::migrations::Migrations;
use crate::services::migrations::triggers::trigger_v0::TriggersV0;

mod trigger_v0;

pub struct TriggerMigration {

}

impl TriggerMigration {

}

impl Migrations for TriggerMigration {
    fn get_current_version() -> usize {
        match TriggersV0::load() {
            None => {}
            Some(_) => { return 0 }
        }

        return usize::MAX
    }

    fn get_latest_version() -> usize {
        return 0
    }

    fn migrate() {
        info!("🚀 Triggers");
        if TriggerMigration::get_current_version() == usize::MAX {
            panic!("❌ Trigger migration failed: Unknown current version")
        }
        if TriggerMigration::get_current_version() == TriggerMigration::get_latest_version() {
            info!("🚀 Trigger migration already up to date")
        }
        if TriggerMigration::get_current_version() > TriggerMigration::get_latest_version() {
            panic!("❌ Trigger migration failed: Outdated bot")
        }

        let mut current_version = TriggerMigration::get_current_version();
        while current_version < TriggerMigration::get_latest_version() {
            match current_version {
                _ => {}
            }
            current_version = current_version + 1;
        }
    }
}