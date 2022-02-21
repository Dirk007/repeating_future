# Crate repeating_future

This crate allows you to make any [Future] to a [futures::Stream] by yielding the futures' result when its ready and calling it over and over again to endlessly produce results.

With the default-enabled feature `streams` this crate provides a handy [FutureStream] which makes a [futures::Stream] out
of any [Future] and streams the results of that [Future] until the underlying future fails.
Look at the [stream-example](https://github.com/Dirk007/repeating_future/blob/master/src/examples/stream.rs) to see how easy life can get. Its a sub-50-lines file which completely converts rumqttc to a [futures::Stream].

With disabled feature `streams` it anyhow defines helpers and a final [RepeatingFuture] that
constructs a [Future] which gets a [Future] from a function of an object and forwards its result. When the underlying function is [Poll::Ready], the function-future is acquired again.
It can be used iE for streaming items from an `async`-function that just delivers one
result per call.

This crate was originally made to extend the terrific [rumqttc](https://docs.rs/rumqttc/0.10.0/rumqttc/) MQTT-client to
support [futures::Stream]. So all examples are made for this for the moment.

## Usage

Add this crate to your Cargo.toml
```
repeating_future = "0.1.0"
```

and use it in your main.rs:
```
use repeating_future::FutureStream;
```

## Easy example
See [Example](examples/stream.rs)
Or for a manual Stream implementation: [Example](examples/manual.rs)

Try `cargo run --example manual` or `cargo run --example stream`