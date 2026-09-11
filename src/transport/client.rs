use std::time::Duration;

use bytes::Bytes;
use reqwest::{Client, StatusCode, header::HeaderMap};

use super::TransportError;

#[derive(Debug)]
pub struct HttpResponse {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Bytes,
}

#[derive(Debug, Clone)]
pub struct TransportClient {
    client: Client,
}

impl TransportClient {
    pub fn new() -> Result<Self, TransportError> {
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(TransportError::ClientBuild)?;

        Ok(Self { client })
    }

    pub async fn get(&self, url: &str) -> Result<HttpResponse, TransportError> {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(TransportError::Request)?;

        Self::collect_response(response).await
    }

    pub async fn post_json<T>(&self, url: &str, body: &T) -> Result<HttpResponse, TransportError>
    where
        T: serde::Serialize + ?Sized,
    {
        let response = self
            .client
            .post(url)
            .json(body)
            .send()
            .await
            .map_err(TransportError::Request)?;

        Self::collect_response(response).await
    }

    async fn collect_response(response: reqwest::Response) -> Result<HttpResponse, TransportError> {
        let status = response.status();
        let headers = response.headers().clone();

        let body = response
            .bytes()
            .await
            .map_err(TransportError::ResponseBody)?;

        Ok(HttpResponse {
            status,
            headers,
            body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_can_be_created() {
        let client = TransportClient::new();

        assert!(client.is_ok());
    }
}
