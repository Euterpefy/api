use axum::{ body::Body, http::{ Request, Response, StatusCode }, response::IntoResponse, Json };
use futures::future::BoxFuture;
use serde::Serialize;
use serde_json::json;
use std::{ convert::Infallible, task::{ Context, Poll } };
use tower_service::Service;
use tower::Layer;

/// A simple struct for error messages to be serialized into JSON.
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

/// A middleware for wrapping responses in JSON format.
#[derive(Clone)]
pub struct JsonResponseWrapperLayer;

impl<S> Layer<S> for JsonResponseWrapperLayer {
    type Service = JsonResponseWrapperService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        JsonResponseWrapperService { inner }
    }
}

#[derive(Clone)]
pub struct JsonResponseWrapperService<S> {
    inner: S,
}

impl<S, B> Service<Request<B>>
    for JsonResponseWrapperService<S>
    where
        S: Service<Request<B>, Response = Response<Body>> + Send + 'static,
        S::Future: Send + 'static,
        B: Send + 'static
{
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // Directly forward the poll_ready result from the inner service
        match self.inner.poll_ready(cx) {
            Poll::Ready(Ok(())) => Poll::Ready(Ok(())),
            Poll::Pending => Poll::Pending,
            // Convert any errors from the inner service into the expected Infallible type
            // Since Infallible cannot actually occur, this branch should ideally never be reached
            // Adjust this as necessary to match your error handling strategy
            Poll::Ready(Err(_)) => {
                // Since Infallible cannot be instantiated, handling this scenario depends on your error strategy.
                // For demonstration, we'll panic, but you should handle this according to your needs.
                panic!("Inner service error occurred, but error type is Infallible.");
            }
        }
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let fut = self.inner.call(req);

        Box::pin(async move {
            match fut.await {
                Ok(resp) => Ok(resp),
                Err(_) => {
                    // Convert errors to a JSON response
                    let error_response = (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(
                            json!(ErrorMessage {
                                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                                message: "Internal server error".to_string(),
                            })
                        ),
                    ).into_response();
                    Ok(error_response)
                }
            }
        })
    }
}

/// Extension trait to simplify applying the JSON response wrapper to an Axum router.
pub trait ApplyJsonResponseWrapperExt {
    fn with_json_response_wrapper(self) -> Self;
}

impl ApplyJsonResponseWrapperExt for axum::Router {
    fn with_json_response_wrapper(self) -> Self {
        self.layer(JsonResponseWrapperLayer)
    }
}
