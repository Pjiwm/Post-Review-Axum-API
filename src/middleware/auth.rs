use axum::http::StatusCode;
use axum::{body::Body, http::Request, response::Response};

use futures::future::BoxFuture;
use std::task::{Context, Poll};
use tower::{Layer, Service};

pub struct AuthLayer;

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for AuthMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request<Body>) -> Self::Future {
        let auth_token = &request.headers().get("authorization");
        println!("{:?}", auth_token);
        let future = self.inner.call(request);
        let _auth_error_response = Response::builder().status(StatusCode::NOT_FOUND).body(());
        let x = Box::pin(async move {
            let response: Response = future.await?;
            Ok(response)
        });
        let _response = Box::pin(Response::builder().status(StatusCode::NOT_FOUND).body(()));
        // TODO fix middleware responding with unauthorized.
        return x;
    }
}
