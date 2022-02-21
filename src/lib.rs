//! This crate allows you to make any [Future] to a [futures::Stream] by yielding the futures' result when its ready and calling it over and over again to endlessly produce results.
//!
//! With the default-enabled feature `streams` this crate provides a handy [FutureStream] which makes a [futures::Stream] out
//! of any [Future] and streams the results of that [Future] until the underlying future fails.
//! Look at the [stream-example](https://github.com/Dirk007/repeating_future/blob/master/src/examples/stream.rs) to see how easy life can get. Its a sub-50-lines file which completely converts rumqttc to a [futures::Stream].
//!
//! With disabled feature `streams` it anyhow defines helpers and a final [RepeatingFuture] that
//! constructs a [Future] which gets a [Future] from a function of an object and forwards its result. When the underlying function is [Poll::Ready], the function-future is acquired again.
//! It can be used iE for streaming items from an `async`-function that just delivers one
//! result per call.
//!
//! This crate was originally made to extend the terrific [rumqttc](https://docs.rs/rumqttc/0.10.0/rumqttc/) MQTT-client to
//! support [futures::Stream]. So all examples are made for this for the moment.

#[cfg(feature = "streams")]
pub mod stream;
use core::{pin::Pin, task::Poll};
use std::ops::{Deref, DerefMut};

use futures_lite::{ready, Future, FutureExt};
pub use stream::FutureStream;

/// Get the future of a function from an object and ecapsulate the object itself
/// with it in a new future. Use [impl_getter]!-macro for an easy
/// implementation. This has to be done to assure the lifetime of the object and
/// its functions future.
pub type GetFutureFunc<T, O> = fn(item: T) -> Pin<Box<dyn Future<Output = (O, T)>>>;

/// Returns a [GetFutureFunc] for the object `object` with function `func`.
/// Func can be any `async` function in `object` returning a [Future].
/// Example: `impl_getter!(rumqttc::EventLoop, poll)`
#[macro_export]
macro_rules! impl_getter {
    ($object:ty, $func:ident) => {
        |mut item: $object| Box::pin(async { (item.$func().await, item) })
    };
}

/// It is used internally here, but left `pub` as maybe somebody could use it
/// in other circumstances.
/// Holds a pinned future of a function from an object and the object itself, guaranteeing by doing so
/// the lifetime of the object and its func-future to co-exist.
/// You need this if you want to poll a [Future] of a function from a object in
/// an async manner. As we hold the object together with the future of its
/// function in a tuple, the object lives at least as long as the future it
/// returns.
pub struct UnderlyingObjectFuture<T, O> {
    left: Option<T>,
    right: Option<Pin<Box<dyn Future<Output = (O, T)>>>>,
    getter: GetFutureFunc<T, O>,
}

impl<T, O> UnderlyingObjectFuture<T, O> {
    /// Create new [UnderlyingObjectFuture] consuming the given `object`.
    /// `getter` can be easily implemented using the [impl_getter] macro and defines
    /// the function-future from the object-function.
    pub fn new(object: T, getter: GetFutureFunc<T, O>) -> Self {
        Self {
            left: Some(object),
            right: None,
            getter,
        }
    }

    /// Get the future from the object and encapsulate the future with the
    /// object or deliver the previous future if the object is already gone.
    pub fn take(&mut self) -> &mut Pin<Box<dyn Future<Output = (O, T)>>> {
        if let Some(left) = self.left.take() {
            self.right = Some((self.getter)(left));
        }

        self.right.as_mut().unwrap()
    }

    /// Reset to object
    pub fn reset(&mut self, left: T) {
        self.left = Some(left);
        self.right = None;
    }
}

/// A [Future] that is holds an object and an async function-call in that object, which can be
/// polled over and over again.
pub struct RepeatingFuture<T, O> {
    owned_future: UnderlyingObjectFuture<T, O>,
}

impl<T, O> RepeatingFuture<T, O> {
    /// Create a new [RepeatingFuture] from object `item` and the built
    /// function call via `getter`. Use the [impl_getter]-macro to easily
    /// implement the [GetFutureFunc].
    pub fn new(item: T, getter: GetFutureFunc<T, O>) -> Self {
        Self {
            owned_future: UnderlyingObjectFuture::new(item, getter),
        }
    }
}

impl<T, O> Future for RepeatingFuture<T, O>
where
    for<'a> Pin<&'a mut Self>: DerefMut + Deref<Target = Self>,
{
    type Output = O;

    fn poll(mut self: Pin<&mut Self>, cx: &mut futures::task::Context<'_>) -> Poll<Self::Output> {
        let future = self.owned_future.take();
        let (result, item) = ready!(future.poll(cx));

        self.owned_future.reset(item);
        Poll::Ready(result)
    }
}
