use crate::request::Request;
use crate::Event;

#[cfg(feature = "streams")]
use crate::StreamRequest;

#[cfg(feature = "streams")]
use tokio_stream::Stream;

/// A mediator is a central hub for communication between components.
pub trait Mediator {
    /// Sends a request to the mediator.
    fn send<Req, Res>(&mut self, req: Req) -> crate::Result<Res>
    where
        Res: 'static,
        Req: Request<Res> + 'static;

    /// Publish an event to the mediator.
    fn publish<E>(&mut self, event: E) -> crate::Result<()>
    where
        E: Event + 'static;

    /// Sends a request to the mediator and returns a stream of responses.
    #[cfg(feature = "streams")]
    fn stream<Req, S, T>(&mut self, req: Req) -> crate::Result<S>
    where
        Req: StreamRequest<Stream = S, Item = T> + 'static,
        S: Stream<Item = T> + 'static,
        T: 'static;
}