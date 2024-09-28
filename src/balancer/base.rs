use color_eyre::Result;
use std::{fmt::Debug, sync::Arc};
use tokio::sync::RwLock;

use super::server::Server;

pub type LoadBalancerType = Arc<RwLock<dyn LoadBalancer<Server> + Send + Sync>>;

#[async_trait::async_trait]
pub trait LoadBalancer<T>
where
    T: Debug + Send,
{
    /// Add a new server to the load balancer
    async fn add_server(&mut self, server: T) -> Result<()>;
    // Maybe instead of string, make it a generic.
    // This will make things more complex, but will make interface more flexible

    /// Remove server from the load balancer
    async fn remove_server(&mut self, key: &str) -> Result<()>;

    /// Retrieve the next server we should be forwarding request to
    async fn next_server(&mut self) -> Result<T>;

    /// Get the number of servers
    async fn server_count(&self) -> usize;
}
