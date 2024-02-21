use scylla::Session;
use scylla::prepared_statement::PreparedStatement;

pub struct ScyllaStorage {
    session: Session,
    keyspace: String,
}

impl ScyllaStorage {
    pub fn new(session: Session, keyspace: String) -> Self {
        ScyllaStorage {
            session,
            keyspace
        }
    }

    pub async fn init(&self) {
        self.create_keyspace().await;
        self.create_tables().await;
    }

    async fn create_keyspace(&self) {
        let query: String = format!("CREATE KEYSPACE IF NOT EXISTS {} WITH REPLICATION = {{ 'class' : 'SimpleStrategy', 'replication_factor' : 1 }}", self.keyspace);

        let statement: PreparedStatement = self.session.prepare(query)
            .await
            .unwrap();

        self.session.execute(&statement, &[])
            .await
            .unwrap();
    }

    async fn create_tables(&self) {
        let queries: [String; 1] = [
            format!("CREATE TABLE IF NOT EXISTS {}.readings (device_id UUID PRIMARY KEY, timestamp TIMESTAMP, reading DECIMAL)", self.keyspace),
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
