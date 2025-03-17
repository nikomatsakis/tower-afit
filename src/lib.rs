#![allow(async_fn_in_trait)]
#![feature(trait_alias)]
#![feature(return_type_notation)]

pub mod filter;

pub trait Service<Request> {
    /// Responses given by the service.
    type Response;

    /// Errors produced by the service.
    type Error;

    /// Process the request and return the response asynchronously.
    ///
    /// This function is expected to be callable off task. As such,
    /// implementations should take care to not call `poll_ready`.
    ///
    /// Before dispatching a request, `poll_ready` must be called and return
    /// `Poll::Ready(Ok(()))`.
    ///
    /// # Panics
    ///
    /// Implementations are permitted to panic if `call` is invoked without
    /// obtaining `Poll::Ready(Ok(()))` from `poll_ready`.
    async fn call(&self, req: Request) -> Result<Self::Response, Self::Error>;
}

pub trait SendService<R> = Service<R, call(..): Send>;

/// Alias for a type-erased error type.
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
