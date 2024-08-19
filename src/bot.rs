use std::sync::Arc;

use anyhow::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::broadcast::Sender;

use crate::{getting_updates::types::UpdateData, utils::ToMultipart};

#[derive(Debug, Clone)]
pub struct Bot {
    pub token: String,
    pub client: reqwest::Client,
    pub sender: Arc<Sender<UpdateData>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CallApiResp {
    pub ok: bool,
    pub result: Option<Value>,
    pub error_code: Option<i64>,
    pub description: Option<String>,
}

impl Bot {
    pub fn new<S: Into<String>>(token: S) -> Bot {
        let (sender, _) = tokio::sync::broadcast::channel(32);
        Bot {
            token: token.into(),
            client: reqwest::Client::new(),
            sender: Arc::new(sender),
        }
    }

    pub(crate) fn format_url(&self, method: &str) -> String {
        format!(
            "https://api.telegram.org/bot{}/{}",
            urlencoding::encode(&self.token),
            method
        )
    }

    pub(crate) async fn call_api_no_payload<D: for<'a> Deserialize<'a> + DeserializeOwned>(
        &self,
        function: &str,
    ) -> Result<D> {
        let url = self.format_url(function);
        let resp = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<CallApiResp>()
            .await?;
        match resp.ok {
            true => match resp.result {
                Some(resp) => Ok(serde_json::from_value(resp)?),
                None => Err(anyhow::anyhow!("Failed to call api: No result")),
            },
            false => {
                let description = resp.description.unwrap_or("No description".to_string());
                Err(anyhow::anyhow!("Failed to call api: {}", description))
            }
        }
    }

    pub(crate) async fn call_api_json<
        D: for<'a> Deserialize<'a> + DeserializeOwned,
        S: Serialize,
    >(
        &self,
        function: &str,
        payload: &S,
    ) -> Result<D> {
        let url = self.format_url(function);
        let resp = self
            .client
            .get(&url)
            .json(payload)
            .send()
            .await?
            .json::<CallApiResp>()
            .await?;
        match resp.ok {
            true => match resp.result {
                Some(resp) => Ok(serde_json::from_value(resp)?),
                None => Err(anyhow::anyhow!("Failed to call api: No result")),
            },
            false => {
                let description = resp.description.unwrap_or("No description".to_string());
                Err(anyhow::anyhow!("Failed to call api: {}", description))
            }
        }
    }

    pub(crate) async fn call_api_multipart<
        D: for<'a> Deserialize<'a> + DeserializeOwned,
        M: ToMultipart,
    >(
        &self,
        function: &str,
        payload: M,
    ) -> Result<D> {
        let url = self.format_url(function);
        let resp = self
            .client
            .post(&url)
            .multipart(payload.to_multipart().await?)
            .send()
            .await?
            .json::<CallApiResp>()
            .await?;

        match resp.ok {
            true => match resp.result {
                Some(resp) => Ok(serde_json::from_value(resp)?),
                None => Err(anyhow::anyhow!("Failed to call api: No result")),
            },
            false => {
                let description = resp.description.unwrap_or("No description".to_string());
                Err(anyhow::anyhow!("Failed to call api: {}", description))
            }
        }
    }
}
