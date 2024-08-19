use serde::{Deserialize, Serialize};

use crate::utils::ToMultipart;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetWebhookPayload {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_token: Option<String>,
}

impl ToMultipart for SetWebhookPayload {
    fn to_multipart(
        self,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = anyhow::Result<reqwest::multipart::Form>>>,
    > {
        Box::pin(async move {
            let mut form = reqwest::multipart::Form::new();
            form = form.text("url", self.url);
            if let Some(c) = self.certificate {
                form = crate::utils::file_to_multipart("certificate".to_string(), c, form).await?;
            }
            if let Some(ip) = self.ip_address {
                form = form.text("ip_address", ip);
            }
            if let Some(mc) = self.max_connections {
                form = form.text("max_connections", mc.to_string());
            }
            if let Some(au) = self.allowed_updates {
                form = form.text("allowed_updates", serde_json::to_string(&au)?);
            }
            if let Some(dpu) = self.drop_pending_updates {
                form = form.text("drop_pending_updates", dpu.to_string());
            }
            if let Some(st) = self.secret_token {
                form = form.text("secret_token", st);
            }
            Ok(form)
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteWebhookPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
}
