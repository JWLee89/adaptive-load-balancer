use color_eyre::eyre::Report;
use thiserror::Error;
/// ## Specifications
///
/// ServerNotFound: Should occur if we cannot find requested server
/// CannotConnectToServer: Occurs when we cannot communicate with target server
/// UnexpectedError: Any other unexpected error
#[derive(Debug, Error)]
pub enum LoadBalancerError {
    #[error("Server not Found")]
    ServerNotFound,
    #[error("Cannot connect to server")]
    CommunicationError,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}
