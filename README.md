# Crate repeating_future

This crate defines helpers and a final [RepeatingFuture] that
constructs a [Future] which gets a [Future] from a function of an object and forwards its result. When the underlying function is [Poll::Ready], the function-future is acquired again.
It can be used iE to [Stream] items from an `async`-function which just delivers one
result per call and could be [Poll::Pending] itself.

This crate was originally made to extend the terrific [rumqttc](https://docs.rs/rumqttc/0.10.0/rumqttc/) MQTT-client to
support [futures::Stream]. Instead of calling the [rumqttc::EventLoop::poll()](https://docs.rs/rumqttc/0.10.0/rumqttc/struct.EventLoop.html#method.poll)
function over and over again, this is a big improvement. (See [exmaple](https://github.com/Dirk007/repeating_future/blob/master/src/examples/ruqttc/main.rs) for an implementation)

## Usage

Add this crate to your Cargo.toml
```
use repeating_future::{impl_getter, RepeatingFuture};
```

## Easy example
See [Example](examples/rumqttc/src/main.rs)
