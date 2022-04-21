// WIP
use axum::{
    async_trait,
    body::{self, BoxBody, Bytes, Full},
    extract::{FromRequest, RequestParts},
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde_json::{json, Value};

use crate::utils;
// middleware that shows how to consume the request body upfront
pub async fn print_request_body(
    request: Request<BoxBody>,
    next: Next<BoxBody>,
) -> Result<impl IntoResponse, Response> {
    let request = buffer_request_body(request).await?;

    Ok(next.run(request).await)
}

// the trick is to take the request apart, buffer the body, do what you need to do, then put
// the request back together
async fn buffer_request_body(request: Request<BoxBody>) -> Result<Request<BoxBody>, Response> {
    let (parts, body) = request.into_parts();
    let uri = &parts.uri;
    let method = &parts.method;
    let auth_token = &parts.headers.get("authorization");

    // this wont work if the body is an long running stream
    let mut bytes = hyper::body::to_bytes(body)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?;
        let bytes_str = std::str::from_utf8(&bytes).unwrap();
        println!("{}", bytes_str);
    do_thing_with_request_body(bytes.clone());

    Ok(Request::from_parts(parts, body::boxed(Full::from(bytes))))
}

fn do_thing_with_request_body(mut bytes: Bytes) {
    println!("{:?}", bytes);
    tracing::debug!(body = ?bytes);
    let mut a = Bytes::from(&b"hello world"[..]);
    let x = String::from("\"password\":").as_bytes();
}

async fn handler(_: PrintRequestBody, body: Bytes) {
    tracing::debug!(?body, "handler received body");
}

// extractor that shows how to consume the request body upfront
struct PrintRequestBody;

#[async_trait]
impl FromRequest<BoxBody> for PrintRequestBody {
    type Rejection = Response;

    async fn from_request(req: &mut RequestParts<BoxBody>) -> Result<Self, Self::Rejection> {
        let request = Request::from_request(req)
            .await
            .map_err(|err| err.into_response())?;

        let request = buffer_request_body(request).await?;

        *req = RequestParts::new(request);

        Ok(Self)
    }
}
