use std::future::Future;

/// A "retry policy" to classify if a request should be retried.
pub trait Policy<Req, Res, E> {
    fn retry(&mut self, req: &mut Req, result: &mut Result<Res, E>) -> Option<impl Future<Output = ()>>;

    /// Tries to clone a request before being passed to the inner service.
    ///
    /// If the request cannot be cloned, return [`None`]. Moreover, the retry
    /// function will not be called if the [`None`] is returned.
    fn clone_request(&self, req: &Req) -> Option<Req>;
}
