use serde::{Serialize, Deserialize};
use uuid::Uuid;
use scylla::macros::{FromRow, SerializeRow};

#[derive(Serialize, Deserialize, Debug, PartialEq, FromRow, SerializeRow)]
pub struct Reading {
    pub device_id: Uuid,
    pub device_timestamp: i64,
    pub reception_timestamp: i64,
    pub measurement_kind: String,
    pub reading: f64,
}
