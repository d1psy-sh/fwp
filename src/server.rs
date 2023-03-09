use std::convert::Infallible;
use std::net::SocketAddr;

use bytes::Bytes;
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use tokio::net::TcpListener;

#[tokio::main]
pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    // server loop
    loop {
        let (stream, _) = listener.accept().await?;

        // Spin up a new task in Tokio so we can continue to listen for new TCP connection on the
        // current task without waiting for the processing of the HTTP1 connection we just received
        // to finish
        tokio::task::spawn(async move {
            // Handle the connection from the client using HTTP1 and pass any
            // HTTP requests received on that connection to the `hello` function
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service_fn(hello_world))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn hello_world(_req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("hello hyper"))))
}
