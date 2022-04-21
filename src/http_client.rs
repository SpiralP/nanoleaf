use log::info;
use reqwest::{header::ACCEPT, Body, Client as ReqwestClient, Response};
use url::Url;

use crate::error::Result;

#[derive(Debug)]
pub(crate) struct HttpClient {
    base_url: Url,
    inner: ReqwestClient,
}

impl HttpClient {
    pub(crate) fn new(base_url: Url, client: ReqwestClient) -> Self {
        HttpClient {
            base_url,
            inner: client,
        }
    }

    pub(crate) async fn post<B: Into<Body>>(&self, path: &str, body: B) -> Result<Response> {
        let url = self.base_url.join(path)?;
        let client = self.inner.clone();

        info!("POST {}", url.as_str());

        Ok(client.post(url.as_str()).body(body.into()).send().await?)
    }

    pub(crate) async fn get(&self, path: &str) -> Result<Response> {
        let url = self.base_url.join(path)?;
        let client = self.inner.clone();

        info!("GET {}", url.as_str());

        Ok(client
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .send()
            .await?)
    }

    pub(crate) async fn delete(&self, path: &str) -> Result<Response> {
        let url = self.base_url.join(path)?;
        let client = self.inner.clone();

        info!("DELETE {}", url.as_str());

        Ok(client.get(url.as_str()).send().await?)
    }

    pub(crate) async fn put<B: Into<Body>>(&self, path: &str, body: B) -> Result<Response> {
        let url = self.base_url.join(path)?;
        let client = self.inner.clone();

        info!("PUT {}", url.as_str());

        Ok(client.put(url.as_str()).body(body.into()).send().await?)
    }
}
