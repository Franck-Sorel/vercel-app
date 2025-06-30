use serde_json::json;
use vercel_runtime::{Body, Error, Request, Response, StatusCode, run};
use url::form_urlencoded;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let query_string = req.uri().query().unwrap_or("");
    let params: HashMap<_, _> = form_urlencoded::parse(query_string.as_bytes()).into_owned().collect();
    let name = params.get("name");
    let number = match name {
        Some(name) => name,
        None => {
            let body = json!({
                "error": "Missing 'name' query parameter"
            })
            .to_string();
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("Content-Type", "application/json")
                .body(body.into())?);
        }
        
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "message": "你好，世界",
              "fib_number": fib_number(number.parse::<u128>().unwrap_or(0)),
            })
            .to_string()
            .into(),
        )?)
}

pub fn fib_number(input: u128 ) -> u128 {
    
    
    
    let mut prev: u128 = 0;
    let mut next: u128 = 1;

    if input == 0 {
        
        return 0;
    }

    let mut number: u128 = 1;
    for _ in 1..input {
        number = prev + next;
        prev = next;
        next = number;
    }
    
    number
}