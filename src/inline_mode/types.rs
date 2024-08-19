use serde::{Deserialize, Serialize};

use crate::{
    available_types::{
        InlineKeyboardMarkup, LinkPreviewOptions, Location, MessageEntity, User, WebAppInfo,
    },
    payments::types::LabeledPrice,
};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,
    pub query: String,
    pub offset: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct AnswerInlineQuery {
    pub inline_query_id: String,
    pub results: Vec<InlineQueryResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_personal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pm_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pm_parameter: Option<String>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct InlineQueryResultsButton {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<String>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: User,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    pub query: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum InlineQueryResult {
    #[serde(rename = "article")]
    Article {
        id: String,
        title: String,
        input_message_content: InputMessageContent,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
        #[serde(skip_serializing_if = "Option::is_none")]
        url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        hide_url: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_width: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_height: Option<i64>,
    },
    #[serde(rename = "photo")]
    Photo {
        id: String,
        photo_url: String,
        thumbnail_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        photo_width: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        photo_height: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        show_caption_above_media: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
    #[serde(rename = "gif")]
    Gif {
        id: String,
        gif_url: String,
        thumbnail_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        gif_width: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gif_height: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gif_duration: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_mime_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        show_caption_above_media: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },

    #[serde(rename = "mpeg4_gif")]
    Mpeg4Gif {
        id: String,
        mpeg4_url: String,
        thumbnail_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        mpeg4_width: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        mpeg4_height: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        mpeg4_duration: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_mime_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        show_caption_above_media: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },

    #[serde(rename = "video")]
    Video {
        id: String,
        video_url: String,
        mime_type: String,
        thumbnail_url: String,
        title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        show_caption_above_media: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        video_width: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        video_height: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        video_duration: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },

    #[serde(rename = "audio")]
    Audio {
        id: String,
        audio_url: String,
        title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        performer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        audio_duration: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },

    #[serde(rename = "voice")]
    Voice {
        id: String,
        voice_url: String,
        title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        voice_duration: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },

    #[serde(rename = "document")]
    Document {
        id: String,
        title: String,
        document_url: String,
        mime_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_width: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_height: Option<i64>,
    },
    #[serde(rename = "location")]
    Location {
        id: String,
        latitude: f64,
        longitude: f64,
        title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        horizontal_accuracy: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        live_period: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        heading: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        proximity_alert_radius: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_width: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_height: Option<i64>,
    },

    #[serde(rename = "venue")]
    Venue {
        id: String,
        latitude: f64,
        longitude: f64,
        title: String,
        address: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        foursquare_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        foursquare_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        google_place_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        google_place_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_width: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_height: Option<i64>,
    },

    #[serde(rename = "contact")]
    Contact {
        id: String,
        phone_number: String,
        first_name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        last_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        vcard: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_width: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_height: Option<i64>,
    },

    #[serde(rename = "game")]
    Game {
        id: String,
        game_short_name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
    },

    #[serde(rename = "photo")]
    CachedPhoto {
        id: String,
        photo_file_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        show_caption_above_media: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },

    #[serde(rename = "gif")]
    CachedGif {
        id: String,
        gif_file_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        show_caption_above_media: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },

    #[serde(rename = "mpeg4_gif")]
    CachedMpeg4Gif {
        id: String,
        mpeg4_file_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        show_caption_above_media: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
    #[serde(rename = "sticker")]
    CachedSticker {
        id: String,
        sticker_file_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
    #[serde(rename = "document")]
    CachedDocument {
        id: String,
        document_file_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        show_caption_above_media: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
    #[serde(rename = "video")]
    CachedVideo {
        id: String,
        video_file_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        show_caption_above_media: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
    #[serde(rename = "voice")]
    CachedVoice {
        id: String,
        voice_file_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        show_caption_above_media: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
    #[serde(rename = "audio")]
    CachedAudio {
        id: String,
        audio_file_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        show_caption_above_media: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    Text {
        message_text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        entities: Option<Vec<MessageEntity>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        link_preview_options: Option<LinkPreviewOptions>,
    },
    Location {
        latitude: f64,
        longitude: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        horizontal_accuracy: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        live_period: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        heading: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        proximity_alert_radius: Option<i64>,
    },
    Venue {
        latitude: f64,
        longitude: f64,
        title: String,
        address: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        foursquare_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        foursquare_type: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        google_place_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        google_place_type: Option<String>,
    },
    Contact {
        phone_number: String,
        first_name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        last_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        vcard: Option<String>,
    },
    Invoice {
        title: String,
        description: String,
        payload: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_token: Option<String>,
        currency: String,
        prices: Vec<LabeledPrice>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max_tip_amount: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        suggested_tip_amounts: Option<Vec<i64>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_data: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        photo_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        photo_size: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        photo_width: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        photo_height: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        need_name: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        need_phone_number: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        need_email: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        need_shipping_address: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        send_phone_number_to_provider: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        send_email_to_provider: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_flexible: Option<bool>,
    },
}
