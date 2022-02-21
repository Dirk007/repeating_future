use std::time::Duration;

use anyhow::Result;
use futures::StreamExt;
use repeating_future::{impl_getter, stream::FutureStream, RepeatingFuture};
use tokio::time::sleep;

#[derive(Default)]
pub struct SecondsCounter {
    value: u64,
}

impl SecondsCounter {
    pub async fn seconds(&mut self) -> u64 {
        sleep(Duration::from_secs(1)).await;
        self.value += 1;

        self.value
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .try_init()
        .ok();

    let counter = SecondsCounter::default();

    let mut stream: FutureStream<_, _> = RepeatingFuture::new(counter, impl_getter!(SecondsCounter, seconds)).into();

    loop {
        match stream.next().await {
            Some(number) => log::info!("Seconds wasted: {}", number),
            None => break,
        }
    }

    Ok(())
}
