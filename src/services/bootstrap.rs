use log::info;
use crate::services::migrations::migrate;

pub fn start() {
    info!("🥾 Bootstrap started...");
    info!("🚀 Migrating:");
    migrate();
}