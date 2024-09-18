use std::sync::Arc;

use adaptive_load_balancer::app::constants::SERVER_URL;
use adaptive_load_balancer::app::state::LoadBalancerApp;
use adaptive_load_balancer::balancer::load_balancer::LoadBalancer;
use adaptive_load_balancer::balancer::round_robin::RoundRobinLoadBalancer;
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
    // TODO: need to update the API. Decouple algorithm from load balancer
    let mut algorithm: RoundRobinLoadBalancer<Server> = RoundRobinLoadBalancer::new();
    algorithm.add_server(Server::new("127.0.0.1:3080")?).await?;
    algorithm.add_server(Server::new("0.0.0.0:8080")?).await?;
    algorithm.add_server(Server::new("0.0.0.0:8080")?).await?;
    let load_balancer: Arc<RwLock<RoundRobinLoadBalancer<_>>> = Arc::new(RwLock::new(algorithm));

    // Listen at target port
    let app = LoadBalancerApp::new(load_balancer);
    app.listen(server_url).await
}
