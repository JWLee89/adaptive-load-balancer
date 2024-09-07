use crate::balancer::load_balancer::LoadBalancerType;

#[derive(Clone)]
pub struct AppState {
    load_balancer: LoadBalancerType,
}

impl AppState {
    pub fn new(load_balancer: LoadBalancerType) -> Self {
        Self { load_balancer }
    }
    pub fn get_load_balancer(&self) -> &LoadBalancerType {
        &self.load_balancer
    }
}
