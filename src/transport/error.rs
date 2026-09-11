use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("failed to build HTTP client: {0}")]
    ClientBuild(#[source] reqwest::Error),

    #[error("HTTP request failed: {0}")]
    Request(#[source] reqwest::Error),

    #[error("failed to read HTTP response body: {0}")]
    ResponseBody(#[source] reqwest::Error),
}
