use uuid::Uuid;

pub mod scylla;

pub trait Storage<T> {
    async fn get_by_id(&self, device_id: &Uuid) -> T;
    async fn get_all(&self) -> Vec<T>;
    async fn create(&self, item: Vec<String>) -> String;
}
