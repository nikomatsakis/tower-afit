use predicate::Predicate;
use futures_util::{future::Either, TryFutureExt};

use crate::{BoxError, Service};

pub mod predicate;

#[derive(Clone, Debug)]
pub struct Filter<T, U> {
    inner: T,
    predicate: U,

}

impl<T, U> Filter<T, U> {
    /// Returns a new [`Filter`] service wrapping `inner`.
    pub const fn new(inner: T, predicate: U) -> Self {
        Self { inner, predicate }
    }
}

impl<T, U, Request> Service<Request> for Filter<T, U>
where
    U: Predicate<Request>,
    T: Service<U::Request>,
    T::Error: Into<BoxError>,
{
    type Response = T::Response;
    type Error = BoxError;

    fn call(&self, request: Request) -> impl Future<Output = Result<T::Response, BoxError>> {
        match self.predicate.check(request) {
            Ok(request) => Either::Right(self.inner.call(request).err_into()),
            Err(e) => Either::Left(futures_util::future::ready(Err(e))),
        }
    }
}
