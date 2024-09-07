use std::sync::Arc;

use adaptive_load_balancer::{
    balancer::{
        load_balancer::{LoadBalancer, RoundRobinLoadBalancer},
        server::Server,
    },
    utils::debug::init_tracing,
};
use color_eyre::eyre::Result;
use hyper::Error;
use tokio::sync::RwLock;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Response;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

// Initialize the application
#[tokio::main]
async fn main() -> Result<()> {
    // Setup tracing:
    // let server_url: &str = SERVER_URL.as_ref();
    color_eyre::install()?;
    init_tracing()?;

    // Create load balancer
    let load_balancer = Arc::new(RwLock::new(RoundRobinLoadBalancer::new()));
    let mut lb = load_balancer.write().await;
    // Add some servers (e.g., server addresses)
    lb.add_server(Server::new("http://localhost:3001")?).await?;
    lb.add_server(Server::new("http://localhost:3002")?).await?;
    lb.add_server(Server::new("http://localhost:3003")?).await?;

    drop(lb);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);
        let load = load_balancer.clone();
        let service = service_fn(move |_req| {
            // Get the current count, and also increment by 1, in a single
            // atomic operation.
            let load = load.clone();
            async move {
                // Will raise error if load balancer cannot retrieve next server
                let next_server = match load.write().await.next_server().await {
                    Ok(s) => s,
                    Err(e) => panic!("Load balancer error: {:?}", e),
                };

                // LGTM. Let's do something with the server :)
                // TODO: Forward the request.
                Ok::<_, Error>(Response::new(Full::new(Bytes::from(format!(
                    "Request: {:?}",
                    next_server
                )))))
            }
        });

        // TODO: refactor later
        if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
            println!("Error serving connection: {:?}", err);
        }
    }
}
