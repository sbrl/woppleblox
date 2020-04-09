use std::pin::Pin;
use std::task::{ Context, Poll };

use actix_service::{ Service, Transform };
use actix_web::{ dev::{ ServiceRequest, ServiceResponse }, Error };

use futures::future;
use futures::Future;

/*
 * TODO: Rip this all out.
 * It looks like the wrap_fn  option allows us to both pre-process *and* post-process.
 * Ref: https://actix.rs/docs/middleware/
 * Look for the bit directly after "Alternatively, for simple use cases, you can use wrap_fn to create small, ad-hoc middlewares"
 */

pub struct MiddlewareLoginBuilder;

impl<NextServiceType, ResponseBodyType> Transform<NextServiceType> for MiddlewareLoginBuilder
where
    NextServiceType: Service<Request = ServiceRequest, Response = ServiceResponse<ResponseBodyType>, Error = Error>,
    NextServiceType::Future: 'static,
    ResponseBodyType: 'static {
    type Request = ServiceRequest;
    type Response = ServiceResponse<ResponseBodyType>;
    type Error = Error;
    type InitError = ();
    type Transform = MiddlewareLogin<NextServiceType>;
    type Future = future::Ready<Result<Self::Transform, Self::InitError>>;
    
    fn new_transform(&self, service: NextServiceType) -> Self::Future {
        future::ok(MiddlewareLogin { service })
    }
}

pub struct MiddlewareLogin<NextServiceType> {
    service: NextServiceType
}

impl<NextServiceType, ResponseBodyType> Service for MiddlewareLogin<NextServiceType>
where
    NextServiceType: Service<Request = ServiceRequest, Response = ServiceResponse<ResponseBodyType>, Error = Error>,
    NextServiceType::Future: 'static,
    ResponseBodyType: 'static {
    type Request = ServiceRequest;
    type Response = ServiceResponse<ResponseBodyType>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
    
    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx);
    }
    
    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        // Do preprocessing here
        
        let promise = self.service.call(req);
        
        // Optional: Do post-processing here. Example:
        Box::pin(async move {
            let response = promise.await?;
            // Post-processing steps go exactly here
            Ok(response);
        })
        
        // Alternatively, return the promise directly if no post-processing is required:
        // promise
    }
}
