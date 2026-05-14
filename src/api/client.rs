use anyhow::Context;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use serde::Serialize;
use serde::de::DeserializeOwned;

pub struct ApiClient {
    client: reqwest::Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(instance: &str, api_key: &str) -> anyhow::Result<Self> {
        let mut headers = HeaderMap::new();
        let auth_value = format!("Bearer {api_key}");
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth_value).context("invalid API key characters")?,
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .context("failed to build HTTP client")?;

        let instance = instance.trim_end_matches('/');
        let base_url = format!("https://{instance}/api");

        Ok(Self { client, base_url })
    }

    fn url(&self, path: &str) -> String {
        format!("{}{path}", self.base_url)
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> anyhow::Result<T> {
        let url = self.url(path);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .with_context(|| format!("GET {url}"))?;

        handle_response(resp, "GET", &url).await
    }

    pub async fn get_query<T: DeserializeOwned, Q: Serialize>(
        &self,
        path: &str,
        query: &Q,
    ) -> anyhow::Result<T> {
        let url = self.url(path);
        let resp = self
            .client
            .get(&url)
            .query(query)
            .send()
            .await
            .with_context(|| format!("GET {url}"))?;

        handle_response(resp, "GET", &url).await
    }

    pub async fn post<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> anyhow::Result<T> {
        let url = self.url(path);
        let resp = self
            .client
            .post(&url)
            .json(body)
            .send()
            .await
            .with_context(|| format!("POST {url}"))?;

        handle_response(resp, "POST", &url).await
    }

    pub async fn put<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> anyhow::Result<T> {
        let url = self.url(path);
        let resp = self
            .client
            .put(&url)
            .json(body)
            .send()
            .await
            .with_context(|| format!("PUT {url}"))?;

        handle_response(resp, "PUT", &url).await
    }

    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> anyhow::Result<T> {
        let url = self.url(path);
        let resp = self
            .client
            .delete(&url)
            .send()
            .await
            .with_context(|| format!("DELETE {url}"))?;

        handle_response(resp, "DELETE", &url).await
    }

    #[allow(dead_code)]
    pub async fn delete_json<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> anyhow::Result<T> {
        let url = self.url(path);
        let resp = self
            .client
            .delete(&url)
            .json(body)
            .send()
            .await
            .with_context(|| format!("DELETE {url}"))?;

        handle_response(resp, "DELETE", &url).await
    }
}

async fn handle_response<T: DeserializeOwned>(
    resp: reqwest::Response,
    method: &str,
    url: &str,
) -> anyhow::Result<T> {
    let status = resp.status();

    if !status.is_success() {
        let body = resp
            .text()
            .await
            .unwrap_or_else(|_| "<unreadable body>".into());

        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&body)
            && let Some(msg) = val.get("message").and_then(|m| m.as_str())
        {
            anyhow::bail!("{status}: {msg}");
        }

        anyhow::bail!("{status}: {body}");
    }

    let body = resp
        .text()
        .await
        .with_context(|| format!("reading response body from {method} {url}"))?;

    serde_json::from_str::<T>(&body)
        .with_context(|| format!("failed to parse response from {method} {url}"))
}
