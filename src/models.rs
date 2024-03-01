use serde::{Serialize, Deserialize};
use uuid::Uuid;
use scylla::macros::{FromRow, SerializeRow};

#[derive(Serialize, Deserialize, Debug, PartialEq, FromRow, SerializeRow)]
pub struct Reading {
    pub device_id: Uuid,
    pub alive: i64,
    pub timestamp: i64,
    pub qualifier: i16,
    pub reading: f64,
}
