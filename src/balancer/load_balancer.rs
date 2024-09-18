use color_eyre::{eyre::eyre, Result};
use std::fmt::Debug;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use super::server::Server;

pub type LoadBalancerType = Arc<RwLock<dyn LoadBalancer<Server> + Send + Sync>>;

#[async_trait::async_trait]
pub trait LoadBalancer<T>: Send
where
    T: Debug + Send,
{
    async fn add_server(&mut self, server: T) -> Result<()>;
    async fn remove_server(&mut self, key: &str) -> Result<()>;
    async fn next_server(&mut self) -> Result<T>;
    async fn server_count(&self) -> usize;
    async fn update_server_load(&mut self, server: &T, delta: isize);
}

#[derive(Debug, PartialEq)]
pub struct RoundRobinLoadBalancer<T>
where
    T: Debug + Send + Clone,
{
    servers: HashMap<String, T>,
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
            servers: HashMap::new(),
            current_index: 0,
        }
    }
}

#[async_trait::async_trait]
impl<T> LoadBalancer<T> for RoundRobinLoadBalancer<T>
where
    T: Debug + Clone + PartialEq + Send + Sync,
{
    async fn add_server(&mut self, server: T) -> Result<()> {
        let key = self.servers.keys().len().to_string();
        if self.servers.contains_key(&key) {
            return Err(eyre!("Server with key {:?} already exists", key));
        }
        self.servers.insert(key, server);
        Ok(())
    }

    async fn remove_server(&mut self, key: &str) -> Result<()> {
        if self.servers.remove(key).is_some() {
            if self.current_index >= self.servers.keys().len() {
                self.current_index = 0;
            }
            Ok(())
        } else {
            Err(eyre!("Server not found"))
        }
    }

    async fn next_server(&mut self) -> Result<T> {
        if self.servers.is_empty() {
            return Err(eyre!("List of target servers is empty"));
        }

        let key = self.current_index.to_string();
        let server = self.servers.get(&key).cloned();

        self.current_index = (self.current_index + 1) % self.servers.keys().len();

        server.ok_or_else(|| eyre!("Server not found"))
    }

    async fn server_count(&self) -> usize {
        self.servers.len()
    }

    async fn update_server_load(&mut self, _server: &T, _delta: isize) {
        // Round Robin does not track server loads.
    }
}
