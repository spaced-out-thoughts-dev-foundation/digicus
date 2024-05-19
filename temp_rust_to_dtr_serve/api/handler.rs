use rust_to_dtr;
use serde::Deserialize;
use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

#[derive(Deserialize)]
struct RequestBody {
    content: String,
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let whole_body = req.body();
    let body = String::from_utf8(whole_body.to_vec())?;

    let parsed: RequestBody = match serde_json::from_str(&body) {
        Ok(parsed) => parsed,
        Err(e) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("Content-Type", "application/json")
                .body(
                    json!({
                        "message": "Invalid request body",
                        "error": e.to_string(),
                    })
                    .to_string()
                    .into(),
                )?)
        }
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
                "message": "你好，世界",
                "body_raw_string": body,
                "content": parsed.content,
                "dtr": rust_to_dtr::parse_to_dtr(&parsed.content)?,
            })
            .to_string()
            .into(),
        )?)
}
