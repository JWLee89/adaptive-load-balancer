use std::sync::Arc;

use adaptive_load_balancer::app::constants::SERVER_URL;
use adaptive_load_balancer::app::state::LoadBalancerApp;
use adaptive_load_balancer::balancer::algorithm::round_robin::RoundRobin;
use adaptive_load_balancer::balancer::base::LoadBalancer;
use adaptive_load_balancer::balancer::basic::BasicLoadBalancer;
use adaptive_load_balancer::{balancer::server::Server, utils::debug::init_tracing};
use color_eyre::eyre::Result;
use tokio::sync::RwLock;

// Initialize the application
#[tokio::main]
async fn main() -> Result<()> {
    // Setup tracing:
    color_eyre::install()?;
    init_tracing()?;
    let server_url: &str = SERVER_URL.as_ref();

    // Create load balancer
    let mut load_balancer = BasicLoadBalancer::builder()
        .algorithm(Box::new(RoundRobin::new()))
        .build();
    load_balancer
        .add_server(Server::new("127.0.0.1:3080")?)
        .await?;
    load_balancer
        .add_server(Server::new("0.0.0.0:8080")?)
        .await?;
    load_balancer
        .add_server(Server::new("0.0.0.0:8080")?)
        .await?;
    let thread_safe_load_balancer = Arc::new(RwLock::new(load_balancer));

    // Listen at target port
    let app = LoadBalancerApp::new(thread_safe_load_balancer);
    app.listen(server_url).await
}
