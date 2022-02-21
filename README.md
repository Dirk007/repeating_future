# Crate repeating_future

This crate allows you to make any 'async'-function to a [futures::Stream] by yielding the function-futures' result when its ready and calling the function over and over again to endlessly get a new future and produce results.

With the default-enabled feature `streams` this crate provides a handy [FutureStream] which makes a [futures::Stream] out
of any 'async'-function and streams the results of that function until the underlying function-future fails.
Look at the [stream-example](https://github.com/Dirk007/repeating_future/blob/master/src/examples/stream.rs) to see how easy life can get. Its a sub-50-lines file which completely converts rumqttc's EventLoop to a [futures::Stream].

With disabled feature `streams` it anyhow defines helpers and a final [RepeatingFuture] that
implements a [Future] which gets a [Future] from an 'async'-function of an object and forwards its result. When the underlying function is [Poll::Ready], the function-future is acquired again.
It can be used iE for streaming items from an `async`-function that just delivers one
result per call.

This crate was originally made to extend the terrific [rumqttc](https://docs.rs/rumqttc/0.10.0/rumqttc/) MQTT-client to
support [futures::Stream].  So most examples are made for this for the moment. 

See the slow_counter example for a minimalistic but totally useless showcase.

## Usage

Add this crate to your Cargo.toml
```
repeating_future = "0.1.0"
```

and use it in your main.rs:
```
use repeating_future::{impl_getter, FutureStream, RepeatingFuturem };
```

## Easy example
See [Example](examples/stream.rs)<br>
Or for a manual Stream implementation: [Example](examples/manual.rs)<br>
Or for a totally useless but "can not be easier" example: [Example](examples/slow_counter.rs)<br>

Try `cargo run --example manual`, `cargo run --example stream` or `cargo run --example slow_counter`