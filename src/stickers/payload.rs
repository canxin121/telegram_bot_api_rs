use std::path::Path;

use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

use crate::{
    available_types::{InlineKeyboardMarkup, ReplyParameters},
    utils::{file_to_multipart, ToMultipart},
};

use super::types::{InputSticker, MaskPosition};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SendStickerPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub sticker: String,
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
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl ToMultipart for SendStickerPayload {
    fn to_multipart<'async_trait>(
        self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = anyhow::Result<Form>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        Self: 'async_trait,
    {
        Box::pin(async move {
            let mut form = reqwest::multipart::Form::new();
            let path = Path::new(&self.sticker);
            if path.is_file() {
                form = file_to_multipart("sticker".to_string(), path, form).await?;
            }else {
                form = form.text("sticker", self.sticker);
            }
            form = form.text("chat_id", self.chat_id);
            if let Some(emoji) = self.emoji {
                form = form.text("emoji", emoji);
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
            if let Some(reply_markup) = self.reply_markup {
                form = form.text("reply_markup", serde_json::to_string(&reply_markup)?);
            }
            Ok(form)
        })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GetStickerSetPayload {
    pub name: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GetCustomEmojiStickersPayload {
    pub custom_emoji_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StickerFormat {
    #[serde(rename = "static")]
    Static,
    #[serde(rename = "animated")]
    Animated,
    #[serde(rename = "video")]
    Video,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadStickerFilePayload {
    pub user_id: i64,
    pub sticker: String,
    pub sticker_format: StickerFormat,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateNewStickerSetPayload {
    pub user_id: i64,
    pub name: String,
    pub title: String,
    pub stickers: Vec<InputSticker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_repainting: Option<bool>,
}

impl ToMultipart for CreateNewStickerSetPayload {
     fn to_multipart<'async_trait>(
        mut self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = anyhow::Result<Form>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        Self: 'async_trait,
    {
        Box::pin(async move {
            let mut form = reqwest::multipart::Form::new();
            for sticker in self.stickers.iter_mut() {
                form = sticker.try_part(form).await?;
            }

            form = form.text("user_id", self.user_id.to_string());
            form = form.text("name", self.name);
            form = form.text("title", self.title);
            if let Some(st) = self.sticker_type {
                form = form.text("sticker_type", st);
            }
            if let Some(nr) = self.needs_repainting {
                form = form.text("needs_repainting", nr.to_string());
            }
            form = form.text("stickers", serde_json::to_string(&self.stickers)?);
            Ok(form)
        })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AddStickerToSetPayload {
    pub user_id: i64,
    pub name: String,
    pub sticker: InputSticker,
}

impl ToMultipart for AddStickerToSetPayload {
    fn to_multipart<'async_trait>(
        self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = anyhow::Result<Form>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        Self: 'async_trait,
    {
        Box::pin(async move {
            let mut form = self.sticker.to_multipart().await?;
            form = form.text("user_id", self.user_id.to_string());
            form = form.text("name", self.name);
            Ok(form)
        })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetStickerPositionInSetPayload {
    pub sticker: String,
    pub position: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeleteStickerFromSetPayload {
    pub sticker: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ReplaceStickerInSetPayload {
    pub user_id: i64,
    pub name: String,
    pub old_sticker: String,
    pub sticker: InputSticker,
}

impl ToMultipart for ReplaceStickerInSetPayload {
    fn to_multipart<'async_trait>(
        self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = anyhow::Result<Form>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        Self: 'async_trait,
    {
        Box::pin(async move {
            let mut form = self.sticker.to_multipart().await?;
            form = form.text("user_id", self.user_id.to_string());
            form = form.text("name", self.name);
            form = form.text("old_sticker", self.old_sticker);
            Ok(form)
        })
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetStickerEmojiListPayload {
    pub sticker: String,
    pub emoji_list: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetStickerKeywordsPayload {
    pub sticker: String,
    pub keywords: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetStickerMaskPositionPayload {
    pub sticker: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetStickerSetTitlePayload {
    pub name: String,
    pub title: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetStickerSetThumbnailPayload {
    pub name: String,
    pub user_id: i64,
    pub thumbnail: String,
    pub format: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SetCustomEmojiStickerSetThumbnailPayload {
    pub name: String,
    pub custom_emoji_id: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeleteStickerSetPayload {
    pub name: String,
}
