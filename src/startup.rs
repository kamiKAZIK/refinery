use rumqttc::{MqttOptions, AsyncClient, Event, Incoming, QoS};
use std::time::Duration;
use scylla::SessionBuilder;

use crate::configuration::Settings;
use crate::messages::Reading;
use crate::storage::scylla::ScyllaStorage;

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
        .build()
        .await
        .unwrap();

    let storage: StorageImpl = StorageImpl::new(session, configuration.scylla.keyspace);
    storage.init().await;

    while let Ok(event) = eventloop.poll().await {
        if let Event::Incoming(Incoming::Publish(packet)) = event {
            println!("Received = {:?}", packet.payload.as_ref());

            match Reading::try_from(packet.payload.as_ref()) {
                Ok(message) => {
                    // let statement = session.prepare("INSERT INTO readings (device_id, timestamp, reading) VALUES (?, ?, ?)").await.unwrap();
                    // session.execute(&statement, (message.device_id, message.timestamp, message.reading)).await.unwrap();

                    println!("Payload = {message:?}")
                },
                Err(error) => println!("Error = {error}"),
            }
        }
    }
}
