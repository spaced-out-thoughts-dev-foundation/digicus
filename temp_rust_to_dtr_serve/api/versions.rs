use rust_to_dtr;
use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "rust_to_dtr_version": rust_to_dtr::version(),
              "temp_rust_to_dtr_version": env!("CARGO_PKG_VERSION")
            })
            .to_string()
            .into(),
        )?)
}
