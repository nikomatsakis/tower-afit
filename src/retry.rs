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
    S: Service<Request>,
{
    type Response = S::Response;
    type Error = S::Error;

    async fn call(&self, mut request: Request) -> Result<Self::Response, Self::Error> {
        let mut policy = self.policy.clone();
        loop {
            match policy.clone_request(&request) {
                Some(cloned_request) => {
                    match self.service.call(cloned_request).await {
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

                None => return self.service.call(request).await,
            }

        }
    }
}
