//! [FutureStream] makes a [futures::Stream] out of any [Future] and streams the futures' results after each other; endlessly.

use core::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::Stream;
use futures_lite::{prelude::*, ready};
use pin_project::pin_project;

use crate::{GetFutureFunc, RepeatingFuture};

/// Streams the given `future` until the underlying future fails
#[pin_project]
pub struct FutureStream<T, O> {
    #[pin]
    future: RepeatingFuture<T, O>,
}

/// Convenience implementation to construct a Stream from an existing RepeatingFuture
impl<T, O> From<RepeatingFuture<T, O>> for FutureStream<T, O> {
    fn from(future: RepeatingFuture<T, O>) -> Self {
        Self { future }
    }
}

impl<T, O> FutureStream<T, O> {
    /// Construct a new [FutureStream] from the given `item` and `getter`
    /// For the getter see [crate::impl_getter] or just use the [From]::[RepeatingFuture].
    /// implementation.
    pub fn new(item: T, getter: GetFutureFunc<T, O>) -> Self {
        Self {
            future: RepeatingFuture::new(item, getter),
        }
    }
}

impl<T, O> Stream for FutureStream<T, O>
where
    RepeatingFuture<T, O>: Future,
{
    type Item = <RepeatingFuture<T, O> as Future>::Output;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();

        let result = ready!(this.future.poll(cx));

        Poll::Ready(Some(result))
    }
}
