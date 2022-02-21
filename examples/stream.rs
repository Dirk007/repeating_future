use std::time::Duration;

use anyhow::Result;
use futures::StreamExt;
use repeating_future::{impl_getter, stream::FutureStream, RepeatingFuture};
use rumqttc::{AsyncClient, Event, EventLoop, MqttOptions, Packet, Publish, QoS, SubAck, SubscribeReasonCode};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .try_init()
        .ok();

    // See https://test.mosquitto.org/
    let mut mqttoptions = MqttOptions::new("function_future_test", "test.mosquitto.org", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(60));

    let (client, event_loop) = AsyncClient::new(mqttoptions, 100);

    client.subscribe("$SYS/#", QoS::AtLeastOnce).await?;

    let future = RepeatingFuture::new(event_loop, impl_getter!(EventLoop, poll));
    let mut stream: FutureStream<_, _> = future.into();

    loop {
        match stream.next().await {
            Some(Err(e)) => {
                log::error!("MQTT error: {:?}. Exiting.", e);
                break;
            }

            Some(Ok(Event::Incoming(Packet::Publish(Publish { topic, payload, .. })))) => {
                log::info!("Received publish: {}: {:?}", topic, payload);
            }
            Some(Ok(Event::Incoming(Packet::SubAck(SubAck { pkid: _, return_codes })))) => {
                if return_codes.iter().any(|code| code == &SubscribeReasonCode::Failure) {
                    log::warn!("Unable to subscribe! Exiting.");
                    break;
                }
            }
            Some(Ok(Event::Outgoing(..))) => {
                // Ignore these - do not spam the console
            }
            Some(unhandled_event) => {
                log::info!("Unhandled event: {:?}", unhandled_event);
            }
            None => {
                log::info!("Stream aborted. Exiting.");
                break;
            }
        }
    }

    Ok(())
}
