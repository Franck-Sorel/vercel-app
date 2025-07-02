use vercel_runtime::{Body, Error, Request, Response, run};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

async fn handler(req: Request) -> Result<Response<Body>, Error> {
    println!("[REQUEST: {:?}]", req);
    println!("[URI: {:?}]", req.uri());
    println!("[PATH: {:?}]", req.uri().path());

    let path = req.uri().path();
    println!("Path of the request... : {path}");
    println!("Path of the request... : {path}");
    // Default behavior on root path

    let doc: &str = r#"{
      "project": "Vercel Rust API",
      "description": "This project exposes two main serverless API endpoints using Vercel and Rust.",
      "routes": [
        {
          "path": "/api",
          "file": "handler.rs",
          "method": "GET",
          "description": "Root endpoint of the API. Returns a welcome message.",
          "example_request": "curl https://my-vercel-api-alpha.vercel.app/api",
          "example_response": "Welcome to my root API"
        },
        {
          "path": "/api/fibbot",
          "file": "fibbot.rs",
          "method": "GET",
          "description": "Computes the Fibonacci number for the value passed through the 'name' query parameter.",
          "query_parameters": [
            {
              "name": "name",
              "type": "integer",
              "required": true,
              "description": "The number for which to calculate the Fibonacci value"
            }
          ],
          "example_request": "curl \"https://my-vercel-api-alpha.vercel.app/api/fibbot?name=10\"",
          "example_response": {
            "message": "你好，世界",
            "fib_number": 55
          },
          "notes": "The 'name' parameter must be a valid non-negative integer, if not i might default on 0"
        }
      ]
    }
    "#;   
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(format!("{}", doc).into())
        .unwrap())
}

