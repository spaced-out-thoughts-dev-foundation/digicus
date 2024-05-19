use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(compile_rust_to_dtr).await
}

pub async fn compile_rust_to_dtr(req: Request) -> Result<Response<Body>, Error> {
    let body = req.body().await?;
    let body = String::from_utf8(body.to_vec())?;
    println!("Request body: {}", body);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "message": "你好，世界"
            })
            .to_string()
            .into(),
        )?)
}