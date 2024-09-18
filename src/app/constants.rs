use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;

lazy_static! {
    pub static ref SERVER_URL: String = get_server_url();
}

/// Retrieve load balancer server url.
/// The URL must also contain the port
/// if it is deemed applicable. E.g.
/// "127.0.0.0:3000"
fn get_server_url() -> String {
    retrieve_dot_env_variable(String::from(env::SERVER_URL))
}

/// Add a variable key
fn retrieve_dot_env_variable(variable_key: String) -> String {
    dotenv().ok();
    let value =
        std_env::var(&variable_key).unwrap_or_else(|_| panic!("{} must be set", variable_key));
    if value.is_empty() {
        panic!("{}", format!("{} must not be empty", variable_key).as_str())
    }
    value
}

pub mod env {
    pub const SERVER_URL: &str = "SERVER_URL";
}
