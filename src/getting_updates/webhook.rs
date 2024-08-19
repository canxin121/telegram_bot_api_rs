use crate::bot::Bot;

use anyhow::Result;

use super::{
    payload::{DeleteWebhookPayload, SetWebhookPayload},
    types::WebhookInfo,
};

impl Bot {
    pub async fn set_webhook(&self, payload: SetWebhookPayload) -> Result<bool> {
        self.call_api_multipart("setWebhook", payload).await
    }
    pub async fn delete_webhook(&self, payload: &DeleteWebhookPayload) -> Result<bool> {
        self.call_api_json("deleteWebhook", payload).await
    }
    pub async fn get_webhook_info(&self) -> Result<WebhookInfo> {
        self.call_api_no_payload("getWebhookInfo").await
    }
}
