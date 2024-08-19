use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SentWebAppMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}
