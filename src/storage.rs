use std::error::Error;

use crate::models::Reading;

pub mod scylla;

pub trait ReadingsRepository {
    async fn create_reading(&self, item: &Reading) -> Result<(), Box<dyn Error>>;
}
