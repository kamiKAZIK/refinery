use crate::models::Reading;

pub mod scylla;

pub trait Storage {
    async fn create_reading(&self, item: Reading);
}
