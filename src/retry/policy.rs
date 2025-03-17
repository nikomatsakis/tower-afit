use std::future::Future;

/// A "retry policy" to classify if a request should be retried.
pub trait Policy<Req, Res, E> {
    fn retry(&self, req: &mut Req, result: &mut Result<Res, E>) -> Option<impl Future<Output = ()>>;

    fn clone_request(&self, req: &Req) -> Option<Req>;
}
