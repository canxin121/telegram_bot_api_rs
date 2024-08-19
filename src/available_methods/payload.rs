use std::path::Path;

use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

use crate::{
    available_types::{
        BotCommand, BotCommandScope, ChatAdministratorRights, ChatPermissions, ForceReply,
        InlineKeyboardMarkup, InputMedia, InputPaidMedia, InputPollOption, LinkPreviewOptions,
        MenuButton, MessageEntity, ReactionType, ReplyKeyboardMarkup, ReplyKeyboardRemove,
        ReplyParameters,
    },
    utils::{file_to_multipart, ToMultipart},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SendMessagePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ForwardMessagePayload {
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub from_chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    pub message_id: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ForwardMessagesPayload {
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub from_chat_id: String,
    pub message_ids: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CopyMessagePayload {
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub from_chat_id: String,
    pub message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CopyMessagesPayload {
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub from_chat_id: String,
    pub message_ids: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_caption: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SendPhotoPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub photo: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl ToMultipart for SendPhotoPayload {
    fn to_multipart(
        self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<Form>>>> {
        Box::pin(async move {
            let mut form = reqwest::multipart::Form::new();
            let path = Path::new(&self.photo);
            if path.is_file() {
                form = file_to_multipart("photo".to_string(), path, form).await?;
            } else {
                form = form.text("photo", self.photo);
            }
            form = form.text("chat_id", self.chat_id);
            if let Some(caption) = self.caption {
                form = form.text("caption", caption);
            }
            if let Some(parse_mode) = self.parse_mode {
                form = form.text("parse_mode", parse_mode);
            }
            if let Some(caption_entities) = &self.caption_entities {
                form = form.text("caption_entities", serde_json::to_string(caption_entities)?);
            }
            if let Some(show_caption_above_media) = &self.show_caption_above_media {
                form = form.text(
                    "show_caption_above_media",
                    show_caption_above_media.to_string(),
                );
            }
            if let Some(has_spoiler) = &self.has_spoiler {
                form = form.text("has_spoiler", has_spoiler.to_string());
            }
            if let Some(disable_notification) = &self.disable_notification {
                form = form.text("disable_notification", disable_notification.to_string());
            }
            if let Some(protect_content) = &self.protect_content {
                form = form.text("protect_content", protect_content.to_string());
            }
            if let Some(message_effect_id) = self.message_effect_id {
                form = form.text("message_effect_id", message_effect_id);
            }
            if let Some(reply_parameters) = &self.reply_parameters {
                form = form.text("reply_parameters", serde_json::to_string(reply_parameters)?);
            }
            if let Some(reply_markup) = &self.reply_markup {
                form = form.text("reply_markup", serde_json::to_string(reply_markup)?);
            }

            Ok(form)
        })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SendAudioPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub audio: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl ToMultipart for SendAudioPayload {
    fn to_multipart(
        self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<Form>>>> {
        Box::pin(async move {
            let mut form = reqwest::multipart::Form::new();
            let path = Path::new(&self.audio);
            if path.is_file() {
                form = file_to_multipart("audio".to_string(), path, form).await?;
            } else {
                form = form.text("audio", self.audio);
            }
            form = form.text("chat_id", self.chat_id);
            if let Some(caption) = self.caption {
                form = form.text("caption", caption);
            }
            if let Some(parse_mode) = self.parse_mode {
                form = form.text("parse_mode", parse_mode);
            }
            if let Some(caption_entities) = &self.caption_entities {
                form = form.text("caption_entities", serde_json::to_string(caption_entities)?);
            }
            if let Some(duration) = &self.duration {
                form = form.text("duration", duration.to_string());
            }
            if let Some(performer) = self.performer {
                form = form.text("performer", performer);
            }
            if let Some(title) = self.title {
                form = form.text("title", title);
            }
            if let Some(thumbnail) = self.thumbnail {
                let path = Path::new(&thumbnail);
                if path.is_file() {
                    form = file_to_multipart("thumbnail".to_string(), path, form).await?;
                } else {
                    form = form.text("thumbnail", thumbnail);
                }
            }
            if let Some(disable_notification) = &self.disable_notification {
                form = form.text("disable_notification", disable_notification.to_string());
            }
            if let Some(protect_content) = &self.protect_content {
                form = form.text("protect_content", protect_content.to_string());
            }
            if let Some(message_effect_id) = self.message_effect_id {
                form = form.text("message_effect_id", message_effect_id);
            }
            if let Some(reply_parameters) = &self.reply_parameters {
                form = form.text("reply_parameters", serde_json::to_string(reply_parameters)?);
            }
            if let Some(reply_markup) = &self.reply_markup {
                form = form.text("reply_markup", serde_json::to_string(reply_markup)?);
            }
            Ok(form)
        })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SendDocumentPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_content_type_detection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl ToMultipart for SendDocumentPayload {
    fn to_multipart(
        self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<Form>>>> {
        Box::pin(async move {
            let mut form = reqwest::multipart::Form::new();
            if let Some(document) = self.document {
                let path = Path::new(&document);
                if path.is_file() {
                    form = file_to_multipart("document".to_string(), path, form).await?;
                } else {
                    form = form.text("document", document);
                }
            }
            if let Some(thumbnail) = self.thumbnail {
                let path = Path::new(&thumbnail);
                if path.is_file() {
                    form = file_to_multipart("thumbnail".to_string(), path, form).await?;
                } else {
                    form = form.text("thumbnail", thumbnail);
                }
            }
            form = form.text("chat_id", self.chat_id);
            if let Some(caption) = self.caption {
                form = form.text("caption", caption);
            }
            if let Some(parse_mode) = self.parse_mode {
                form = form.text("parse_mode", parse_mode);
            }
            if let Some(caption_entities) = &self.caption_entities {
                form = form.text("caption_entities", serde_json::to_string(caption_entities)?);
            }
            if let Some(disable_content_type_detection) = &self.disable_content_type_detection {
                form = form.text(
                    "disable_content_type_detection",
                    disable_content_type_detection.to_string(),
                );
            }
            if let Some(disable_notification) = &self.disable_notification {
                form = form.text("disable_notification", disable_notification.to_string());
            }
            if let Some(protect_content) = &self.protect_content {
                form = form.text("protect_content", protect_content.to_string());
            }
            if let Some(message_effect_id) = self.message_effect_id {
                form = form.text("message_effect_id", message_effect_id);
            }
            if let Some(reply_parameters) = &self.reply_parameters {
                form = form.text("reply_parameters", serde_json::to_string(reply_parameters)?);
            }
            if let Some(reply_markup) = &self.reply_markup {
                form = form.text("reply_markup", serde_json::to_string(reply_markup)?);
            }
            Ok(form)
        })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SendVideoPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl ToMultipart for SendVideoPayload {
    fn to_multipart(
        self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<Form>>>> {
        Box::pin(async move {
            let mut form = reqwest::multipart::Form::new();
            if let Some(video) = self.video {
                let path = Path::new(&video);
                if path.is_file() {
                    form = file_to_multipart("video".to_string(), path, form).await?;
                } else {
                    form = form.text("video", video);
                }
            }
            if let Some(thumbnail) = self.thumbnail {
                let path = Path::new(&thumbnail);
                if path.is_file() {
                    form = file_to_multipart("thumbnail".to_string(), path, form).await?;
                } else {
                    form = form.text("thumbnail", thumbnail);
                }
            }
            form = form.text("chat_id", self.chat_id);
            if let Some(caption) = self.caption {
                form = form.text("caption", caption);
            }
            if let Some(parse_mode) = self.parse_mode {
                form = form.text("parse_mode", parse_mode);
            }
            if let Some(caption_entities) = &self.caption_entities {
                form = form.text("caption_entities", serde_json::to_string(caption_entities)?);
            }
            if let Some(show_caption_above_media) = &self.show_caption_above_media {
                form = form.text(
                    "show_caption_above_media",
                    show_caption_above_media.to_string(),
                );
            }
            if let Some(has_spoiler) = &self.has_spoiler {
                form = form.text("has_spoiler", has_spoiler.to_string());
            }
            if let Some(supports_streaming) = &self.supports_streaming {
                form = form.text("supports_streaming", supports_streaming.to_string());
            }
            if let Some(disable_notification) = &self.disable_notification {
                form = form.text("disable_notification", disable_notification.to_string());
            }
            if let Some(protect_content) = &self.protect_content {
                form = form.text("protect_content", protect_content.to_string());
            }
            if let Some(message_effect_id) = self.message_effect_id {
                form = form.text("message_effect_id", message_effect_id);
            }
            if let Some(reply_parameters) = &self.reply_parameters {
                form = form.text("reply_parameters", serde_json::to_string(reply_parameters)?);
            }
            if let Some(reply_markup) = &self.reply_markup {
                form = form.text("reply_markup", serde_json::to_string(reply_markup)?);
            }
            Ok(form)
        })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SendAnimationPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl ToMultipart for SendAnimationPayload {
    fn to_multipart(
        self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<Form>>>> {
        Box::pin(async move {
            let mut form = reqwest::multipart::Form::new();
            if let Some(animation) = self.animation {
                let path = Path::new(&animation);
                if path.is_file() {
                    form = file_to_multipart("animation".to_string(), path, form).await?;
                } else {
                    form = form.text("animation", animation);
                }
            }
            if let Some(thumbnail) = self.thumbnail {
                let path = Path::new(&thumbnail);
                if path.is_file() {
                    form = file_to_multipart("thumbnail".to_string(), path, form).await?;
                } else {
                    form = form.text("thumbnail", thumbnail);
                }
            }
            form = form.text("chat_id", self.chat_id);
            if let Some(caption) = self.caption {
                form = form.text("caption", caption);
            }
            if let Some(parse_mode) = self.parse_mode {
                form = form.text("parse_mode", parse_mode);
            }
            if let Some(caption_entities) = &self.caption_entities {
                form = form.text("caption_entities", serde_json::to_string(caption_entities)?);
            }
            if let Some(show_caption_above_media) = &self.show_caption_above_media {
                form = form.text(
                    "show_caption_above_media",
                    show_caption_above_media.to_string(),
                );
            }
            if let Some(has_spoiler) = &self.has_spoiler {
                form = form.text("has_spoiler", has_spoiler.to_string());
            }
            if let Some(disable_notification) = &self.disable_notification {
                form = form.text("disable_notification", disable_notification.to_string());
            }
            if let Some(protect_content) = &self.protect_content {
                form = form.text("protect_content", protect_content.to_string());
            }
            if let Some(message_effect_id) = self.message_effect_id {
                form = form.text("message_effect_id", message_effect_id);
            }
            if let Some(reply_parameters) = &self.reply_parameters {
                form = form.text("reply_parameters", serde_json::to_string(reply_parameters)?);
            }
            if let Some(reply_markup) = &self.reply_markup {
                form = form.text("reply_markup", serde_json::to_string(reply_markup)?);
            }
            Ok(form)
        })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SendVoicePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl ToMultipart for SendVoicePayload {
    fn to_multipart(
        self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<Form>>>> {
        Box::pin(async move {
            let mut form = reqwest::multipart::Form::new();
            if let Some(voice) = self.voice {
                let path = Path::new(&voice);
                if path.is_file() {
                    form = file_to_multipart("voice".to_string(), path, form).await?;
                } else {
                    form = form.text("voice", voice);
                }
            }
            form = form.text("chat_id", self.chat_id);
            if let Some(caption) = self.caption {
                form = form.text("caption", caption);
            }
            if let Some(parse_mode) = self.parse_mode {
                form = form.text("parse_mode", parse_mode);
            }
            if let Some(caption_entities) = &self.caption_entities {
                form = form.text("caption_entities", serde_json::to_string(caption_entities)?);
            }
            if let Some(duration) = &self.duration {
                form = form.text("duration", duration.to_string());
            }
            if let Some(disable_notification) = &self.disable_notification {
                form = form.text("disable_notification", disable_notification.to_string());
            }
            if let Some(protect_content) = &self.protect_content {
                form = form.text("protect_content", protect_content.to_string());
            }
            if let Some(message_effect_id) = self.message_effect_id {
                form = form.text("message_effect_id", message_effect_id);
            }
            if let Some(reply_parameters) = &self.reply_parameters {
                form = form.text("reply_parameters", serde_json::to_string(reply_parameters)?);
            }
            if let Some(reply_markup) = &self.reply_markup {
                form = form.text("reply_markup", serde_json::to_string(reply_markup)?);
            }
            Ok(form)
        })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SendVideoNotePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl ToMultipart for SendVideoNotePayload {
    fn to_multipart(
        self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<Form>>>> {
        Box::pin(async move {
            let mut form = reqwest::multipart::Form::new();
            if let Some(video_note) = self.video_note {
                let path = Path::new(&video_note);
                if path.is_file() {
                    form = file_to_multipart("video_note".to_string(), path, form).await?;
                } else {
                    form = form.text("video_note", video_note);
                }
            }
            if let Some(thumbnail) = self.thumbnail {
                let path = Path::new(&thumbnail);
                if path.is_file() {
                    form = file_to_multipart("thumbnail".to_string(), path, form).await?;
                } else {
                    form = form.text("thumbnail", thumbnail);
                }
            }
            form = form.text("chat_id", self.chat_id);
            if let Some(duration) = &self.duration {
                form = form.text("duration", duration.to_string());
            }
            if let Some(length) = &self.length {
                form = form.text("length", length.to_string());
            }
            if let Some(disable_notification) = &self.disable_notification {
                form = form.text("disable_notification", disable_notification.to_string());
            }
            if let Some(protect_content) = &self.protect_content {
                form = form.text("protect_content", protect_content.to_string());
            }
            if let Some(message_effect_id) = self.message_effect_id {
                form = form.text("message_effect_id", message_effect_id);
            }
            if let Some(reply_parameters) = &self.reply_parameters {
                form = form.text("reply_parameters", serde_json::to_string(reply_parameters)?);
            }
            if let Some(reply_markup) = &self.reply_markup {
                form = form.text("reply_markup", serde_json::to_string(reply_markup)?);
            }
            Ok(form)
        })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SendPaidMediaPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: String,
    pub star_count: i64,
    pub media: Vec<InputPaidMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl ToMultipart for SendPaidMediaPayload {
    fn to_multipart(
        mut self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<Form>>>> {
        Box::pin(async move {
            let mut form = reqwest::multipart::Form::new();
            for media in self.media.iter_mut() {
                form = media.try_part(form).await?;
            }

            form = form.text("media", serde_json::to_string(&self.media)?);
            form = form.text("chat_id", self.chat_id);
            form = form.text("star_count", self.star_count.to_string());
            if let Some(caption) = &self.caption {
                form = form.text("caption", caption.to_string());
            }
            if let Some(parse_mode) = &self.parse_mode {
                form = form.text("parse_mode", parse_mode.to_string());
            }
            if let Some(caption_entities) = &self.caption_entities {
                form = form.text("caption_entities", serde_json::to_string(caption_entities)?);
            }
            if let Some(show_caption_above_media) = &self.show_caption_above_media {
                form = form.text(
                    "show_caption_above_media",
                    show_caption_above_media.to_string(),
                );
            }
            if let Some(disable_notification) = &self.disable_notification {
                form = form.text("disable_notification", disable_notification.to_string());
            }
            if let Some(protect_content) = &self.protect_content {
                form = form.text("protect_content", protect_content.to_string());
            }
            if let Some(reply_parameters) = &self.reply_parameters {
                form = form.text("reply_parameters", serde_json::to_string(reply_parameters)?);
            }
            if let Some(reply_markup) = &self.reply_markup {
                form = form.text("reply_markup", serde_json::to_string(reply_markup)?);
            }
            Ok(form)
        })
    }
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SendMediaGroupPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub media: Vec<InputMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
}

impl ToMultipart for SendMediaGroupPayload {
    fn to_multipart(
        mut self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<Form>>>> {
        Box::pin(async move {
            let mut form = reqwest::multipart::Form::new();
            for media in self.media.iter_mut() {
                form = media.try_part(form).await?;
            }
            form = form.text("media", serde_json::to_string(&self.media)?);
            form = form.text("chat_id", self.chat_id);
            if let Some(message_thread_id) = self.message_thread_id {
                form = form.text("message_thread_id", message_thread_id.to_string());
            }
            if let Some(disable_notification) = self.disable_notification {
                form = form.text("disable_notification", disable_notification.to_string());
            }
            if let Some(protect_content) = self.protect_content {
                form = form.text("protect_content", protect_content.to_string());
            }
            if let Some(message_effect_id) = self.message_effect_id {
                form = form.text("message_effect_id", message_effect_id);
            }
            if let Some(reply_parameters) = self.reply_parameters {
                form = form.text(
                    "reply_parameters",
                    serde_json::to_string(&reply_parameters)?,
                );
            }
            Ok(form)
        })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SendLocationPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SendVenuePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SendContactPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub phone_number: String,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SendPollPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub question: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_entities: Option<Vec<MessageEntity>>,
    pub options: Vec<InputPollOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_multiple_answers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_closed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SendDicePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SendChatActionPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub action: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetMessageReactionPayload {
    pub chat_id: String,
    pub message_id: i64,
    pub reaction: Vec<ReactionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_big: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GetUserProfilePhotosPayload {
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GetFilePayload {
    pub file_id: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BanChatMemberPayload {
    pub chat_id: String,
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_messages: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UnbanChatMemberPayload {
    pub chat_id: String,
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_if_banned: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RestrictChatMemberPayload {
    pub chat_id: String,
    pub user_id: i64,
    pub permissions: ChatPermissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_independent_chat_permissions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PromoteChatMemberPayload {
    pub chat_id: String,
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_chat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_video_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_restrict_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_promote_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_stories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_stories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_stories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetChatAdministratorCustomTitlePayload {
    pub chat_id: String,
    pub user_id: i64,
    pub custom_title: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BanChatSenderPayload {
    pub chat_id: String,
    pub sender_chat_id: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UnbanChatSenderPayload {
    pub chat_id: String,
    pub sender_chat_id: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetChatPermissionsPayload {
    pub chat_id: String,
    pub permissions: ChatPermissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_independent_chat_permissions: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ExportChatInviteLinkPayload {
    pub chat_id: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateChatInviteLinkPayload {
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EditChatInviteLinkPayload {
    pub chat_id: String,
    pub invite_link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateChatSubscriptionInviteLinkPayload {
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub subscription_period: i64,
    pub subscription_price: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EditChatSubscriptionInviteLinkPayload {
    pub chat_id: String,
    pub invite_link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RevokeChatInviteLinkPayload {
    pub chat_id: String,
    pub invite_link: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ApproveChatJoinRequestPayload {
    pub chat_id: String,
    pub user_id: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeclineChatJoinRequestPayload {
    pub chat_id: String,
    pub user_id: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetChatPhotoPayload {
    pub chat_id: String,
    pub photo: String,
}

impl ToMultipart for SetChatPhotoPayload {
    fn to_multipart(
        self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<Form>>>> {
        Box::pin(async move {
            let mut form = reqwest::multipart::Form::new();
            let path = Path::new(&self.photo);
            if path.is_file() {
                form = file_to_multipart("photo".to_string(), path, form).await?;
            } else {
                form = form.text("photo", self.photo);
            }
            Ok(form)
        })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeleteChatPhotoPayload {
    pub chat_id: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetChatTitlePayload {
    pub chat_id: String,
    pub title: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetChatDescriptionPayload {
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PinChatMessagePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: String,
    pub message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UnpinChatMessagePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ChatIdPayload {
    pub chat_id: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GetChatMemberPayload {
    pub chat_id: String,
    pub user_id: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetChatStickerSetPayload {
    pub chat_id: String,
    pub sticker_set_name: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeleteChatStickerSetPayload {
    pub chat_id: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateForumTopicPayload {
    pub chat_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_color: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EditForumTopicPayload {
    pub chat_id: String,
    pub message_thread_id: i64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ForumTopicPayload {
    pub chat_id: String,
    pub message_thread_id: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EditGeneralForumTopicPayload {
    pub chat_id: String,
    pub name: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AnswerCallbackQueryPayload {
    pub callback_query_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alert: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GetUserChatBoostsPayload {
    pub chat_id: String,
    pub user_id: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GetBusinessConnectionPayload {
    pub business_connection_id: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetMyCommandsPayload {
    pub commands: Vec<BotCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeleteMyCommandsPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GetMyCommandsPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetMyNamePayload {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LanguageCodePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetMyDescriptionPayload {
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetMyShortDescriptionPayload {
    pub short_description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetChatMenuButtonPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_button: Option<MenuButton>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetMyDefaultAdministratorRightsPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<ChatAdministratorRights>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GetMyDefaultAdministratorRightsPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<bool>,
}
