use adaptive_load_balancer::utils::debug::init_tracing;

// Initialize the test application
#[tokio::main]
async fn main() {
    color_eyre::install().unwrap();
    init_tracing().unwrap();

    // Create apps that have the following endpoints
    // TODO: create tests later
    let _ = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
}
