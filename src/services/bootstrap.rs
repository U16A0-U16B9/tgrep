use log::info;
use crate::services::environment_variables;
use crate::services::migrations::migrate;

pub fn start() {
    pretty_env_logger::init();
    info!("🥾 Bootstrap started...");
    info!("⚙️ Upsetting environment:");
    environment_variables::load();
    info!("⚙️ Environment set");
    info!("🚀 Migrating:");
    migrate();
    info!("🚀 Migration completed");
}