use color_eyre::eyre::eyre;
use color_eyre::Result;
use std::sync::Arc;
use std::{collections::HashMap, fmt::Debug};
use tokio::sync::RwLock;

use super::algorithm::base::LoadBalancingAlgorithm;
use super::algorithm::round_robin::RoundRobin;
use super::base::LoadBalancer;

#[derive(Debug)]
pub struct BasicLoadBalancer<T>
where
    T: Debug + Send + Clone + 'static,
{
    servers: Arc<RwLock<HashMap<usize, T>>>,
    algorithm: Box<dyn LoadBalancingAlgorithm<T> + 'static>,
}

impl<T> Default for BasicLoadBalancer<T>
where
    T: Debug + PartialEq + Send + Sync + Clone,
{
    fn default() -> Self {
        Builder::new().build()
    }
}

pub struct Builder<T> {
    servers: Arc<RwLock<HashMap<usize, T>>>,
    algorithm: Box<dyn LoadBalancingAlgorithm<T>>,
}

impl<T> Default for Builder<T>
where
    T: Debug + PartialEq + Send + Sync + Clone + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Builder<T>
where
    T: Debug + PartialEq + Send + Sync + Clone + 'static,
{
    pub fn new() -> Self {
        let servers = Arc::new(RwLock::new(HashMap::new()));
        Self {
            servers: servers.clone(),
            algorithm: Box::new(RoundRobin::new()),
        }
    }
    pub fn algorithm(mut self, algorithm: Box<dyn LoadBalancingAlgorithm<T>>) -> Self {
        self.algorithm = algorithm;
        self
    }
    pub fn build(self) -> BasicLoadBalancer<T> {
        BasicLoadBalancer {
            servers: self.servers,
            algorithm: self.algorithm,
        }
    }
}

impl<T> BasicLoadBalancer<T>
where
    T: Debug + PartialEq + Send + Sync + Clone,
{
    pub fn builder() -> Builder<T> {
        Builder::new()
    }
}

#[async_trait::async_trait]
impl<T> LoadBalancer<T> for BasicLoadBalancer<T>
where
    T: Debug + Clone + PartialEq + Send + Sync,
{
    #[tracing::instrument("Add server", skip_all)]
    async fn add_server(&mut self, server: T) -> Result<()> {
        let mut servers = self.servers.write().await;
        let key = servers.len();
        if servers.contains_key(&key) {
            return Err(eyre!("Server with key {:?} already exists", key));
        }
        servers.insert(key, server);
        Ok(())
    }

    #[tracing::instrument("Remove server", skip_all)]
    async fn remove_server(&mut self, key: &str) -> Result<()> {
        let key_usize = key.parse::<usize>()?;
        let mut servers = self.servers.write().await;
        if servers.remove(&key_usize).is_some() {
            Ok(())
        } else {
            Err(eyre!("Server not found"))
        }
    }

    #[tracing::instrument("Retrieve next server", skip_all)]
    async fn next_server(&mut self) -> Result<T> {
        self.algorithm.next(&self.servers).await
    }

    #[tracing::instrument("Get number of servers", skip_all)]
    async fn server_count(&self) -> usize {
        self.servers.read().await.len()
    }
}
