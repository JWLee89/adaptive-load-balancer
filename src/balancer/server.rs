use color_eyre::eyre::{eyre, Result};
use std::fmt::Display;
use url::Url;

#[derive(Debug, Clone, PartialEq)]
pub struct Server {
    address: String,
}

impl Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Address: {}", self.address)
    }
}

impl Server {
    pub fn new(address: &str) -> Result<Self> {
        let current_url =
            Url::parse(address).map_err(|_| eyre!("Cannot parse invalid URL: {:?}", address))?;
        Ok(Self {
            address: current_url.to_string(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("http://127.0.0.1:3456")]
    #[test_case("http://google.com")]
    #[test_case("http://192.168.0.234")]
    #[tokio::test]
    pub async fn server_equality(address: &str) {
        let server_a = Server::new(address);
        if let Ok(a) = server_a {
            let server_b = Server::new(address).unwrap();
            assert_eq!(a, server_b);
        } else {
            panic!("The developer screwed up with writing the test")
        }
    }

    #[test_case("http://127.0.0.1:3456", "http://121.0.2.1:3456")]
    #[test_case("http://google.com:3456", "http://google.com")]
    #[tokio::test]
    pub async fn test_server_inequality(source: &str, target: &str) {
        let server_a = Server::new(source).unwrap();
        let server_b = Server::new(target).unwrap();
        assert_ne!(server_a, server_b);
    }
}
