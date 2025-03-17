use hyper::{Body, Request, Response};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::fs;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match req.method() {
        // Handle GET requests
        &hyper::Method::GET => {
            let path = req.uri().path();
            if path == "/" {
                // Serve static file for the root URL
                match fs::read_to_string("index.html").await {
                    Ok(content) => Ok(Response::new(Body::from(content))),
                    Err(_) => Ok(Response::builder()
                        .status(404)
                        .body(Body::from("File not found"))
                        .unwrap()),
                }
            } else {
                // Return 404 for other paths
                Ok(Response::builder()
                    .status(404)
                    .body(Body::from("Not Found"))
                    .unwrap())
            }
        }
        // Handle POST requests
        &hyper::Method::POST => {
            let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let body_string = String::from_utf8_lossy(&body_bytes);

            // Respond with the received data
            Ok(Response::new(Body::from(format!(
                "Received POST request with body: {}",
                body_string
            ))))
        }
        // Handle unsupported HTTP methods
        _ => Ok(Response::builder()
            .status(405)
            .body(Body::from("Method Not Allowed"))
            .unwrap()),
    }
}

#[tokio::main]
async fn main() {
    // Define the server address (localhost:3000)
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Listening on http://{}", addr);

    // Create a TCP listener
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind to address");

    // Create HTTP connection handler
    let http = hyper::server::conn::Http::new();

    // Accept and serve connections in a loop
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                // Clone Http for each connection (it's cheap)
                let http = http.clone();
                
                // Spawn a task to serve this connection
                tokio::spawn(async move {
                    if let Err(e) = http
                        .serve_connection(stream, service_fn(handle_request))
                        .await
                    {
                        eprintln!("Connection error: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}