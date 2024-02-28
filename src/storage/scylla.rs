use scylla::Session;
use scylla::prepared_statement::PreparedStatement;

use crate::models::Reading;
use crate::storage::Storage;

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
        let queries: [String; 1] = [
            String::from("CREATE TABLE IF NOT EXISTS readings (device_id UUID PRIMARY KEY, device_timestamp TIMESTAMP, reception_timestamp TIMESTAMP, measurement_kind TEXT, reading DECIMAL)"),
        ];

        for query in queries {
            let statement: PreparedStatement = self.session.prepare(query)
                .await
                .unwrap();

            self.session.execute(&statement, &[])
                .await
                .unwrap();
        }
    }
}

impl Storage for ScyllaStorage {
    async fn create_reading(&self, item: Reading) {
        let statement: PreparedStatement = self.session.prepare("INSERT INTO readings (device_id, device_timestamp, reception_timestamp, measurement_kind, reading) VALUES (?, ?, ?, ?, ?)")
            .await
            .unwrap();

        self.session.execute(&statement, &item)
            .await
            .unwrap();
    }
}
