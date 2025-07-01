use vercel_runtime::{Body, Error, Request, Response, run};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let path = req.uri().path();

    match path {
        "/" => {
            // Default behavior on root path
            Ok(Response::builder()
                .status(200)
                .header("content-type", "text/plain")
                .body("Welcome to my API root!".into())
                .unwrap())
        }
        "/api/some_endpoint" => {
            // Handle other endpoints
            Ok(Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(r#"{"message":"some_endpoint hit"}"#.into())
                .unwrap())
        }
        _ => {
            // Return 404 or a custom error for undefined paths
            Ok(Response::builder()
                .status(404)
                .header("content-type", "text/plain")
                .body("Not found".into())
                .unwrap())
        }
    }
}
