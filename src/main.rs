use log::info;

use crate::configuration::Settings;

mod configuration;
mod startup;
mod messages;
mod models;
mod storage;

#[tokio::main]
async fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    
    let settings = Settings::new().unwrap();

    info!("Starting refinery service...");

    startup::run(settings).await;
}
