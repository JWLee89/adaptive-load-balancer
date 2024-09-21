use color_eyre::Result;
use std::fmt::Debug;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[async_trait::async_trait]
pub trait LoadBalancingAlgorithm<T>: Send + Sync {
    async fn next(&mut self, server_map: &Arc<RwLock<HashMap<usize, T>>>) -> Result<T>;
}

impl<T> Debug for dyn LoadBalancingAlgorithm<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Load Balancing Algorithm")
    }
}
