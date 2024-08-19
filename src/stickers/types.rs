use std::path::Path;

use anyhow::{Ok, Result};
use rand::Rng;
use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};

use crate::{
    available_types::{File, PhotoSize},
    utils::{file_to_multipart, ToMultipart},
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MaskPosition {
    pub point: String,
    pub x_shift: f64,
    pub y_shift: f64,
    pub scale: f64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Sticker {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i64,
    pub height: i64,
    pub is_animated: bool,
    pub is_video: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_animation: Option<File>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_repainting: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StickerSet {
    pub name: String,
    pub title: String,
    pub sticker_type: String,
    pub stickers: Vec<Sticker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    pub a: MaskPosition,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InputSticker {
    pub sticker: String,
    pub format: String,
    pub emoji_list: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
    pub keywords: Vec<String>,
}

impl InputSticker {
    pub async fn try_part(&mut self, form: Form) -> Result<Form> {
        let path = Path::new(&self.sticker);
        if !path.is_file() {
            return Ok(form);
        }
        let name = path
            .file_name()
            .and_then(|f| f.to_str())
            .and_then(|f| Some(f.to_string()))
            .unwrap_or_else(|| {
                // 生成一个16位的随机字符串
                let mut rng = rand::thread_rng();
                let mut name = String::with_capacity(16);
                for _ in 0..16 {
                    name.push(rng.gen_range(b'a'..b'z') as char);
                }
                name
            });

        let form = file_to_multipart(name.clone(), self.sticker.clone(), form).await?;
        self.sticker = format!("attach://{}", name);

        Ok(form)
    }
}

impl ToMultipart for InputSticker {
    fn to_multipart(
        self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<reqwest::multipart::Form>>>>
    {
        Box::pin(async move {
            let mut form = reqwest::multipart::Form::new();
            let path = Path::new(&self.sticker);
            if path.is_file() {
                form = file_to_multipart("sticker".to_string(), self.sticker, form).await?;
            } else {
                form = form.text("sticker", self.sticker);
            }
            form = form.text("format", self.format);
            form = form.text("emoji_list", serde_json::to_string(&self.emoji_list)?);
            if let Some(mp) = self.mask_position {
                form = form.text("mask_position", serde_json::to_string(&mp)?);
            }
            form = form.text("keywords", serde_json::to_string(&self.keywords)?);
            Ok(form)
        })
    }
}
