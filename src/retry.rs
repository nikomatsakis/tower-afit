use policy::Policy;

use crate::Service;

mod policy;

#[derive(Clone, Debug)]
pub struct Retry<P, S> {
    policy: P,
    service: S,
}

impl<P, S> Retry<P, S> {
    /// Retry the inner service depending on this [`Policy`].
    pub const fn new(policy: P, service: S) -> Self {
        Retry { policy, service }
    }
}

impl<P, S, Request> Service<Request> for Retry<P, S>
where
    P: Policy<Request, S::Response, S::Error> + Clone,
    S: Service<Request> + Clone,
{
    type Response = S::Response;
    type Error = S::Error;

    async fn call(&self, mut request: Request) -> Result<Self::Response, Self::Error> {
        loop {
            match service.call(request.clone()).await {
                Ok(response) => return Ok(response),
                mut response @ Err(_) => {
                    if let Some(duration) = policy.retry(&mut request, &mut response) {
                        duration.await;
                        continue;
                    }

                    return response;
                }
            }
        }
    }
}
