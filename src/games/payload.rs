use serde::{Deserialize, Serialize};

use crate::available_types::{InlineKeyboardMarkup, ReplyParameters};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendGamePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub game_short_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetGameScorePayload {
    pub user_id: i64,
    pub score: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_edit_message: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGameHighScoresPayload {
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}
