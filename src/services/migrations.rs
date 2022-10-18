use triggers::TriggerMigration;
mod triggers;

pub trait Migrations {
    fn get_current_version() -> usize;
    fn get_latest_version() -> usize;
    fn migrate();
}

pub fn migrate() {
    TriggerMigration::migrate();
}
