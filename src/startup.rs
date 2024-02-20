use rumqttc::{MqttOptions, AsyncClient, Event, Incoming, QoS};
use std::time::Duration;

use crate::configuration::Settings;
use crate::messages::Reading;

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

    while let Ok(event) = eventloop.poll().await {
        if let Event::Incoming(Incoming::Publish(packet)) = event {
            println!("Received = {:?}", packet.payload.as_ref());

            match Reading::try_from(packet.payload.as_ref()) {
                Ok(message) => println!("Payload = {message:?}"),
                Err(error) => println!("Error = {error}"),
            }
        }
    }
}
