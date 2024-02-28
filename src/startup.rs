use rumqttc::{MqttOptions, AsyncClient, Event, Incoming, QoS};
use std::time::{Duration, Instant};
use scylla::SessionBuilder;

use crate::configuration::Settings;
use crate::messages;
use crate::models;
use crate::storage::scylla::ScyllaStorage;
use crate::storage::Storage;

type StorageImpl = ScyllaStorage;

pub async fn run(configuration: Settings) {
    let mut mqtt_options: MqttOptions = MqttOptions::new(
        &configuration.mqtt.client_id,
        &configuration.mqtt.host,
        configuration.mqtt.port
    );
    mqtt_options
        .set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);

    client
        .subscribe("hello/rumqtt", QoS::AtMostOnce)
        .await
        .unwrap();

    let session = SessionBuilder::new()
        .known_node(&configuration.scylla.uri)
        .user(&configuration.scylla.user, &configuration.scylla.password)
        .use_keyspace(&configuration.scylla.keyspace, false)
        .build()
        .await
        .unwrap();

    let storage: StorageImpl = StorageImpl::new(session);
    storage.init()
        .await;

    while let Ok(event) = eventloop.poll().await {
        if let Event::Incoming(Incoming::Publish(packet)) = event {
            println!("Received = {:?}", packet.payload.as_ref());

            match messages::Reading::try_from(packet.payload.as_ref()) {
                Ok(message) => {
                    let item: models::Reading = models::Reading{
                        device_id: message.device_id,
                        device_timestamp: message.timestamp as i64,
                        reception_timestamp: Instant::now().elapsed().as_secs() as i64,
                        measurement_kind: String::from("HUM"),
                        reading: message.reading,
                    };

                    storage.create_reading(item)
                        .await;
                },
                Err(error) => println!("Error = {error}"),
            }
        }
    }
}
