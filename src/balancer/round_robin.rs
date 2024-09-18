use color_eyre::eyre::eyre;
use color_eyre::Result;
use std::sync::Arc;
use std::{collections::HashMap, fmt::Debug};
use tokio::sync::RwLock;

use super::load_balancer::LoadBalancer;

#[derive(Debug)]
pub struct RoundRobinLoadBalancer<T>
where
    T: Debug + Send + Clone,
{
    servers: Arc<RwLock<HashMap<usize, T>>>,
    current_index: usize,
}

impl<T> Default for RoundRobinLoadBalancer<T>
where
    T: Debug + PartialEq + Send + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> RoundRobinLoadBalancer<T>
where
    T: Debug + PartialEq + Send + Clone,
{
    pub fn new() -> Self {
        Self {
            servers: Arc::new(RwLock::new(HashMap::new())),
            current_index: 0,
        }
    }
}

#[async_trait::async_trait]
impl<T> LoadBalancer<T> for RoundRobinLoadBalancer<T>
where
    T: Debug + Clone + PartialEq + Send + Sync,
{
    #[tracing::instrument("Add server Round Robin", skip_all)]
    async fn add_server(&mut self, server: T) -> Result<()> {
        let mut servers = self.servers.write().await;
        let key = servers.len();
        if servers.contains_key(&key) {
            return Err(eyre!("Server with key {:?} already exists", key));
        }
        servers.insert(key, server);
        Ok(())
    }

    #[tracing::instrument("Add server Round Robin", skip_all)]
    async fn remove_server(&mut self, key: &str) -> Result<()> {
        let key_usize = key.parse::<usize>()?;
        let mut servers = self.servers.write().await;
        if servers.remove(&key_usize).is_some() {
            if self.current_index >= servers.len() {
                self.current_index = 0;
            }
            Ok(())
        } else {
            Err(eyre!("Server not found"))
        }
    }

    #[tracing::instrument("Retrieve next server Round Robin", skip_all)]
    async fn next_server(&mut self) -> Result<T> {
        let servers = self.servers.read().await;
        if servers.is_empty() {
            return Err(eyre!("List of target servers is empty"));
        }
        let server = servers.get(&self.current_index).cloned();

        self.current_index = (self.current_index + 1) % servers.len();

        server.ok_or_else(|| eyre!("Server not found"))
    }

    #[tracing::instrument("Get number of servers for round robin", skip_all)]
    async fn server_count(&self) -> usize {
        self.servers.read().await.len()
    }
}
