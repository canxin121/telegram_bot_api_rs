pub mod payload;
pub mod types;
pub mod webhook;

use serde::{Deserialize, Serialize};
use tokio::sync::broadcast::Receiver;
use types::{AllowedUpdateType, Update, UpdateData};

use crate::bot::Bot;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetUpdatesResponse {
    ok: bool,
    #[serde(default)]
    result: Vec<Update>,
    error_code: Option<i64>,
    description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetUpdateConfig {
    pub limit: u8,
    pub timeout: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<AllowedUpdateType>>,
}

impl Default for GetUpdateConfig {
    fn default() -> Self {
        Self {
            limit: 100,
            timeout: 60,
            offset: None,
            allowed_updates: None,
        }
    }
}

impl Bot {
    pub fn subscribe_updates(&self) -> Receiver<UpdateData> {
        self.sender.subscribe()
    }

    pub fn start_get_updates(&self, mut config: GetUpdateConfig) {
        let url = self.format_url("getUpdates");
        let client = self.client.clone();
        let mut last_update_id: Option<i64> = None;
        let sender = self.sender.clone();

        tokio::spawn(async move {
            loop {
                if let Some(id) = last_update_id {
                    config.offset = Some(id + 1);
                }

                let response = client.post(&url).json(&config).send().await;

                match response {
                    Ok(response) => match response.json::<GetUpdatesResponse>().await {
                        Ok(response) => match response.ok {
                            true => {
                                for update in response.result {
                                    if let Err(e) = sender.send(update.data) {
                                        tracing::error!(
                                            "Failed to send update to subscriber: {}",
                                            e
                                        );
                                    }
                                    last_update_id = Some(update.update_id);
                                }
                            }
                            false => {
                                tracing::error!(
                                    "Failed to get updates, Code: {}, Description: {}",
                                    response.error_code.unwrap_or(0),
                                    response.description.unwrap_or("No description".to_string())
                                );
                            }
                        },
                        Err(e) => {
                            tracing::error!("Failed to parse getUpdates response: {}", e);
                            continue;
                        }
                    },
                    Err(e) => {
                        tracing::error!("Failed to get updates: {}", e);
                        continue;
                    }
                }
            }
        });
    }
}
