//! # mediator-rs
//! An implementation of the Mediator pattern in Rust
//! inspired in C# [MediatR](https://github.com/jbogard/MediatR/tree/master/src/MediatR).
//!
//! ## Mediator Pattern
//! https://en.wikipedia.org/wiki/Mediator_pattern
//!
//! ## Example
//! ```rust
//! use mediator::{DefaultMediator, Mediator, Request, Event, RequestHandler, EventHandler};
//!
//! #[derive(Clone, Debug)]
//! enum Op {
//!  Add(f32, f32),
//!  Sub(f32, f32),
//!  Mul(f32, f32),
//!  Div(f32, f32),
//! }
//!
//! struct MathRequest(Op);
//! impl Request<Option<f32>> for MathRequest {}
//!
//! #[derive(Clone, Debug)]
//! struct MathEvent(Op, Option<f32>);
//! impl Event for MathEvent {}
//!
//! struct MathRequestHandler(DefaultMediator);
//! impl RequestHandler<MathRequest, Option<f32>> for MathRequestHandler {
//!     fn handle(&mut self, req: MathRequest) -> Option<f32> {
//!         let result = match req.0 {
//!             Op::Add(a, b) => Some(a + b),
//!             Op::Sub(a, b) => Some(a - b),
//!             Op::Mul(a, b) => Some(a * b),
//!             Op::Div(a, b) => {
//!                 if b == 0.0 { None } else { Some(a / b) }
//!             }
//!         };
//!
//!         self.0.publish(MathEvent(req.0, result));
//!         result
//!     }
//! }
//!
//! fn main() {
//!     let mut mediator = DefaultMediator::builder()
//!         .add_handler_deferred(|m| MathRequestHandler(m))
//!         .subscribe_fn(|event: MathEvent| {
//!            println!("{:?}", event);
//!          })
//!         .build();
//! }
//! ```

/// A convenient result type.
pub type Result<T> = std::result::Result<T, error::Error>;

/// Module for the mediator request-response.
mod request;
pub use request::*;

/// Module for the mediator events.
mod event;
pub use event::*;

/// Module for the errors.
mod error;
pub use error::*;

/// Module for the mediator.
mod mediator;
pub use crate::mediator::*;

/// Provides default implementations.
#[cfg(feature = "impls")]
mod impls;

#[cfg(feature = "impls")]
pub use impls::*;
