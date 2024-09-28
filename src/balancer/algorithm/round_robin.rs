use color_eyre::eyre::eyre;
use color_eyre::Result;
use std::fmt::Debug;
use std::{collections::HashMap, marker::PhantomData, sync::Arc};
use tokio::sync::RwLock;

use super::base::LoadBalancingAlgorithm;

#[derive(Debug)]
pub struct RoundRobin<T> {
    _a: PhantomData<T>,
    current_index: usize,
}

impl<T> Default for RoundRobin<T> {
    fn default() -> Self {
        Self {
            _a: Default::default(),
            current_index: Default::default(),
        }
    }
}

impl<T> RoundRobin<T> {
    pub fn new() -> Self {
        Self {
            _a: PhantomData,
            current_index: 0,
        }
    }
}

#[async_trait::async_trait]
impl<T> LoadBalancingAlgorithm<T> for RoundRobin<T>
where
    T: Clone + Send + Sync,
{
    async fn next(&mut self, server_map: &Arc<RwLock<HashMap<usize, T>>>) -> Result<T> {
        let servers = server_map.write().await;

        if servers.is_empty() {
            return Err(eyre!("List of target servers is empty"));
        }

        let server = servers.get(&self.current_index).cloned();
        self.current_index = (self.current_index + 1) % servers.len();

        server.ok_or_else(|| eyre!("Server not found"))
    }
}
