use std::time::{Duration, SystemTime};

use uuid::Uuid;

pub struct Reading {
    pub device_id: Uuid,
    pub alive: Duration,
    pub timestamp: SystemTime,
    pub qualifier: u8,
    pub reading: f64,
}
