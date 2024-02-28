mod configuration;
mod startup;
mod messages;
mod models;
mod storage;

use crate::configuration::Settings;

#[tokio::main]
async fn main() {
    let settings: Settings = Settings::new().unwrap();

    startup::run(settings).await;
}
