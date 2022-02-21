use core::{
    pin::Pin,
    task::{Context, Poll},
};
use std::time::Duration;

use anyhow::Result;
use futures::Stream;
use futures_lite::{prelude::*, ready};
use pin_project::pin_project;
use repeating_future::{impl_getter, RepeatingFuture};
use rumqttc::{
    AsyncClient, ConnectionError, Event, EventLoop, MqttOptions, Packet, Publish, QoS, SubAck, SubscribeReasonCode,
};

/// Wrapper around [rumqttc::EventLoop] and its poll() method
#[pin_project]
pub struct MqttStream {
    #[pin]
    event_future: RepeatingFuture<EventLoop, Result<Event, ConnectionError>>,
}

impl MqttStream {
    pub fn new(event_loop: EventLoop) -> Self {
        Self {
            event_future: RepeatingFuture::new(event_loop, impl_getter!(EventLoop, poll)),
        }
    }
}

impl Stream for MqttStream {
    type Item = Event;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        let event = ready!(this.event_future.poll(cx));

        match event {
            Ok(event) => Poll::Ready(Some(event)),
            Err(_e) => Poll::Ready(None),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .try_init()
        .ok();

    // See https://test.mosquitto.org/
    let mut mqttoptions = MqttOptions::new("function_future_test", "test.mosquitto.org", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(60));

    let (client, event_loop) = AsyncClient::new(mqttoptions, 100);
    let mut stream = MqttStream::new(event_loop);

    client.subscribe("$SYS/#", QoS::AtLeastOnce).await?;

    loop {
        match stream.next().await {
            Some(Event::Incoming(Packet::Publish(Publish { topic, payload, .. }))) => {
                log::info!("Received publish: {}: {:?}", topic, payload);
            }
            Some(Event::Incoming(Packet::SubAck(SubAck { pkid: _, return_codes }))) => {
                if return_codes.iter().any(|code| code == &SubscribeReasonCode::Failure) {
                    log::warn!("Unable to subscribe! Exiting.");
                    break;
                }
            }
            Some(Event::Outgoing(..)) => {
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
