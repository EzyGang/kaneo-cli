use std::time::Duration;

use anyhow::Context;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue, RETRY_AFTER};
use reqwest::{Method, Response, StatusCode};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::api::error::{ApiError, ErrorBody};

/// Requests sent to a rate-limited API key before the error reaches the user.
const MAX_ATTEMPTS: u32 = 3;
/// Longest `Retry-After` the client waits for instead of failing; the server's window is a minute.
const MAX_RETRY_WAIT: Duration = Duration::from_secs(60);

/// An authenticated HTTP client for the Kaneo REST API.
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

    pub async fn get<T>(&self, path: &str) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
    {
        self.send(Method::GET, path, None::<&()>, None::<&()>).await
    }

    pub async fn get_query<T, Q>(&self, path: &str, query: &Q) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        self.send(Method::GET, path, Some(query), None::<&()>).await
    }

    pub async fn post<B, T>(&self, path: &str, body: &B) -> Result<T, ApiError>
    where
        B: Serialize,
        T: DeserializeOwned,
    {
        self.send(Method::POST, path, None::<&()>, Some(body)).await
    }

    pub async fn put<B, T>(&self, path: &str, body: &B) -> Result<T, ApiError>
    where
        B: Serialize,
        T: DeserializeOwned,
    {
        self.send(Method::PUT, path, None::<&()>, Some(body)).await
    }

    pub async fn put_query<B, T, Q>(&self, path: &str, query: &Q, body: &B) -> Result<T, ApiError>
    where
        B: Serialize,
        T: DeserializeOwned,
        Q: Serialize,
    {
        self.send(Method::PUT, path, Some(query), Some(body)).await
    }

    pub async fn patch<B, T>(&self, path: &str, body: &B) -> Result<T, ApiError>
    where
        B: Serialize,
        T: DeserializeOwned,
    {
        self.send(Method::PATCH, path, None::<&()>, Some(body))
            .await
    }

    pub async fn delete<T>(&self, path: &str) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
    {
        self.send(Method::DELETE, path, None::<&()>, None::<&()>)
            .await
    }

    /// Sends one request, retrying while the API key is rate limited.
    async fn send<T, Q, B>(
        &self,
        method: Method,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
    ) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
        Q: Serialize,
        B: Serialize,
    {
        let url = format!("{}{path}", self.base_url);
        let mut attempt = 1;
        loop {
            let mut request = self.client.request(method.clone(), &url);
            if let Some(query) = query {
                request = request.query(query);
            }
            if let Some(body) = body {
                request = request.json(body);
            }
            let response = request.send().await.map_err(|source| ApiError::Transport {
                url: url.clone(),
                source,
            })?;

            match retry_delay(&response) {
                Some(wait) if attempt < MAX_ATTEMPTS => {
                    crate::output::warn(&format!(
                        "Rate limited by the server, retrying in {}s",
                        wait.as_secs()
                    ));
                    tokio::time::sleep(wait).await;
                    attempt += 1;
                }
                _ => return decode(response, &url).await,
            }
        }
    }
}

/// Returns how long to wait before retrying a rate-limited response.
///
/// A missing `Retry-After` means the usage quota never refills, so the request is not retried.
fn retry_delay(response: &Response) -> Option<Duration> {
    if response.status() != StatusCode::TOO_MANY_REQUESTS {
        return None;
    }
    let seconds: u64 = response
        .headers()
        .get(RETRY_AFTER)?
        .to_str()
        .ok()?
        .trim()
        .parse()
        .ok()?;
    let wait = Duration::from_secs(seconds.max(1));
    (wait <= MAX_RETRY_WAIT).then_some(wait)
}

async fn decode<T>(response: Response, url: &str) -> Result<T, ApiError>
where
    T: DeserializeOwned,
{
    let status = response.status();
    let text = response
        .text()
        .await
        .map_err(|source| ApiError::Transport {
            url: url.to_owned(),
            source,
        })?;

    if !status.is_success() {
        return Err(ApiError::Status {
            status,
            body: ErrorBody::parse(status, &text),
        });
    }

    let text = if text.trim().is_empty() {
        "null"
    } else {
        &text
    };
    serde_json::from_str(text).map_err(|source| ApiError::Decode {
        url: url.to_owned(),
        source,
    })
}
