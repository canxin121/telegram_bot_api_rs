use serde::{Deserialize, Serialize};

use crate::available_types::{InlineKeyboardMarkup, InputMedia, LinkPreviewOptions, MessageEntity};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EditMessageTextPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EditMessageCaptionPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    pub caption: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditMessageMediaPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    pub media: InputMedia,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EditMessageLiveLocationPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StopMessageLiveLocationPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EditMessageReplyMarkupPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StopPollPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: String,
    pub message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeleteMessagePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: String,
    pub message_id: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeleteMessagesPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: String,
    pub message_ids: Vec<i64>,
}
