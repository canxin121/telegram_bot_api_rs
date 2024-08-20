pub mod payload;
pub mod types;

use crate::{
    available_types::{File, Message},
    bot::Bot,
};

use anyhow::Result;
use payload::*;
use types::{Sticker, StickerSet};

impl Bot {
    pub async fn send_sticker(&self, payload: SendStickerPayload) -> Result<Message> {
        self.call_api_multipart("sendSticker", payload).await
    }

    pub async fn get_sticker_set(&self, payload: &GetStickerSetPayload) -> Result<StickerSet> {
        self.call_api_json("GetStickerSet", payload).await
    }

    pub async fn get_custom_emoji_stickers(
        &self,
        payload: &GetCustomEmojiStickersPayload,
    ) -> Result<Vec<Sticker>> {
        self.call_api_json("GetCustomEmojiStickers", payload).await
    }

    pub async fn upload_sticker_file(&self, payload: &UploadStickerFilePayload) -> Result<File> {
        self.call_api_json("UploadStickerFile", payload).await
    }

    pub async fn create_new_sticker_set(
        &self,
        payload: CreateNewStickerSetPayload,
    ) -> Result<bool> {
        self.call_api_multipart("CreateNewStickerSet", payload)
            .await
    }

    pub async fn add_sticker_to_set(&self, payload: AddStickerToSetPayload) -> Result<bool> {
        self.call_api_multipart("AddStickerToSet", payload).await
    }

    pub async fn set_sticker_position_in_set(
        &self,
        payload: &SetStickerPositionInSetPayload,
    ) -> Result<bool> {
        self.call_api_json("SetStickerPositionInSet", payload).await
    }

    pub async fn delete_sticker_from_set(
        &self,
        payload: &DeleteStickerFromSetPayload,
    ) -> Result<bool> {
        self.call_api_json("DeleteStickerFromSet", payload).await
    }

    pub async fn replace_sticker_in_set(
        &self,
        payload: ReplaceStickerInSetPayload,
    ) -> Result<bool> {
        self.call_api_multipart("ReplaceStickerInSet", payload)
            .await
    }

    pub async fn set_sticker_emoji_list(
        &self,
        payload: &SetStickerEmojiListPayload,
    ) -> Result<bool> {
        self.call_api_json("SetStickerEmojiList", payload).await
    }

    pub async fn set_sticker_keywords(&self, payload: &SetStickerKeywordsPayload) -> Result<bool> {
        self.call_api_json("SetStickerKeywords", payload).await
    }

    pub async fn set_sticker_mask_position(
        &self,
        payload: &SetStickerMaskPositionPayload,
    ) -> Result<bool> {
        self.call_api_json("SetStickerMaskPosition", payload).await
    }

    pub async fn set_sticker_set_title(&self, payload: &SetStickerSetTitlePayload) -> Result<bool> {
        self.call_api_json("SetStickerSetTitle", payload).await
    }

    pub async fn set_sticker_set_thumbnail(
        &self,
        payload: &SetStickerSetThumbnailPayload,
    ) -> Result<bool> {
        self.call_api_json("SetStickerSetThumbnail", payload).await
    }

    pub async fn set_custom_emoji_sticker_set_thumbnail(
        &self,
        payload: &SetCustomEmojiStickerSetThumbnailPayload,
    ) -> Result<bool> {
        self.call_api_json("SetCustomEmojiStickerSetThumbnail", payload)
            .await
    }

    pub async fn delete_sticker_set(&self, payload: &DeleteStickerSetPayload) -> Result<bool> {
        self.call_api_json("DeleteStickerSet", payload).await
    }
}
