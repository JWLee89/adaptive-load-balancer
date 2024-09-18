use std::sync::Arc;

use crate::balancer::{load_balancer::LoadBalancerType, round_robin::RoundRobinLoadBalancer};
use color_eyre::{eyre::eyre, Result};
use hyper::{body::Incoming, server::conn::http1, service::service_fn, Request, Response};
use hyper_util::rt::TokioIo;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::RwLock,
};

#[derive(Clone)]
pub struct LoadBalancerApp {
    load_balancer: LoadBalancerType,
}

impl Default for LoadBalancerApp {
    fn default() -> Self {
        LoadBalancerApp {
            load_balancer: Arc::new(RwLock::new(RoundRobinLoadBalancer::default())),
        }
    }
}

impl LoadBalancerApp {
    pub fn new(load_balancer: LoadBalancerType) -> Self {
        Self { load_balancer }
    }

    // App will continue to listen at the given url
    // for requests
    #[tracing::instrument("Listen", skip_all)]
    pub async fn listen(&self, addr: &str) -> Result<()> {
        let listener = TcpListener::bind(addr).await?;
        // We start a loop to continuously accept incoming connections
        loop {
            let (stream, _) = listener.accept().await?;

            // Use an adapter to access something implementing `tokio::io` traits as if they implement
            // `hyper::rt` IO traits.
            let io = TokioIo::new(stream);
            let service = service_fn(move |req: hyper::Request<hyper::body::Incoming>| {
                async move {
                    // Will raise error if load balancer cannot retrieve next server
                    self.forward_request(req).await
                }
            });

            // TODO: refactor later
            let connection = http1::Builder::new().serve_connection(io, service).await;
            if connection.is_err() {
                // DO something
                // TODO: add callback to allow users to handle their own errors
                println!("Error: cannot process connection: {:?}", connection);
            }
        }
    }

    pub fn get_load_balancer(&self) -> &LoadBalancerType {
        &self.load_balancer
    }

    #[tracing::instrument("Forward http request", skip_all)]
    pub async fn forward_request(
        &self,
        req: hyper::Request<hyper::body::Incoming>,
    ) -> Result<Response<Incoming>> {
        // Retrieve next server to forward to
        let server = self.get_load_balancer().write().await.next_server().await?;
        let url = server.uri;
        let host = url
            .host()
            .ok_or_else(|| eyre!("Uri has not host: {}", url))?;
        let port = url.port_u16().unwrap_or(80);
        let addr: String = format!("{}:{}", host, port);

        // Connect to that server.
        // Alternatively, a keep-alive feature could be useful if requests are frequent.
        // We can add that as a future feature.
        let stream = TcpStream::connect(&addr)
            .await
            .map_err(|_| eyre!("Failed to connect to: {:?}", &addr))?;
        let io = TokioIo::new(stream);

        // Check whether connection is alive.
        let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                println!("Connection failed: {:?}", err);
            }
        });

        // Extract the headers from the original request
        let headers = req.headers().clone();

        // Clone the original request's headers and method
        let mut new_req = Request::builder()
            .method(req.method())
            .uri(url)
            .body(req.into_body())?;

        // Copy headers from the original request
        for (key, value) in headers.iter() {
            new_req.headers_mut().insert(key, value.clone());
        }
        println!("Sending request to: {:?}", &addr);

        sender
            .send_request(new_req)
            .await
            .map_err(|_| eyre!("HTTP Request Error"))
    }
}
