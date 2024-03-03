use std::error::Error;
use std::ops::{Div, Rem};
use std::time::UNIX_EPOCH;

use scylla::frame::value::{CqlDuration, CqlTimestamp};
use scylla::{FromRow, SerializeRow, Session};
use uuid::Uuid;

use crate::models::Reading;
use crate::storage::ReadingsRepository;

pub struct ScyllaStorage {
    session: Session,
}

impl ScyllaStorage {
    pub fn new(session: Session) -> Self {
        ScyllaStorage {
            session,
        }
    }

    pub async fn init(&self) {
        self.create_tables().await;
    }

    async fn create_tables(&self) {
        let queries = [
            "CREATE TABLE IF NOT EXISTS readings (device_id UUID PRIMARY KEY, alive DURATION, timestamp TIMESTAMP, qualifier SMALLINT, reading DOUBLE)",
        ];

        for query in queries {
            let prepared_statement = self.session.prepare(query);
            
            match prepared_statement.await {
                Ok(statement) => {
                    self.session.execute(&statement, &[])
                        .await
                },
                Err(err) => {
                    Err(err)
                },
            }.unwrap();
        }
    }
}

#[derive(Debug, FromRow, SerializeRow)]
struct ReadingRow {
    device_id: Uuid,
    alive: CqlDuration,
    timestamp: CqlTimestamp,
    qualifier: i16,
    reading: f64,
}

impl From<Reading> for ReadingRow {
    fn from(value: Reading) -> Self {
        ReadingRow {
            device_id: value.device_id,
            alive: CqlDuration {
                months: 0,
                days: value.alive.as_secs().div(24 * 60 * 60) as i32,
                nanoseconds: value.alive.as_nanos().rem(1000000 * 60 * 60 * 24) as i64
            },
            timestamp: CqlTimestamp(value.timestamp.duration_since(UNIX_EPOCH).unwrap().as_millis() as i64),
            qualifier: value.qualifier as i16,
            reading: value.reading,
        }
    }
}

impl ReadingsRepository for ScyllaStorage {
    async fn create_reading(&self, item: Reading) -> Result<(), Box<dyn Error>> {
        let prepared_statement = self.session.prepare("INSERT INTO readings (device_id, alive, timestamp, qualifier, reading) VALUES (?, ?, ?, ?, ?)");
        
        match prepared_statement.await {
            Ok(statement) => {
                self.session.execute(&statement, &ReadingRow::from(item))
                    .await
                    .map(|_| ())
                    .map_err(|err| err.into())
            },
            Err(err) =>{
                Err(err.into())
            },
        }
    }
}
