#![allow(non_snake_case)]
use std::path::Path;

use crate::stickers::types::Sticker;
use crate::utils::file_to_multipart;
use anyhow::Result;
use rand::Rng as _;
use serde::{de, Deserialize, Deserializer, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_to_attachment_menu: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_join_groups: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read_all_group_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_inline_queries: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_connect_to_business: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_main_web_app: Option<bool>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Chat {
    pub id: i64,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<bool>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ChatPhoto {
    pub small_file_id: String,
    pub small_file_unique_id: String,
    pub big_file_id: String,
    pub big_file_unique_id: String,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Birthdate {
    pub day: i64,
    pub month: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i64>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PhotoSize {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i64,
    pub height: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct File {
    pub file_id: String,
    pub file_unique_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
}

impl File {
    pub fn get_file_path(&self, token: &str) -> String {
        format!(
            "https://api.telegram.org/file/bot{}/{}",
            token,
            self.file_path.as_ref().unwrap()
        )
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Location {
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
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BusinessLocation {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BusinessOpeningHoursInterval {
    pub opening_minute: i64,
    pub closing_minute: i64,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BusinessOpeningHours {
    pub time_zone_name: String,
    pub opening_hours: Vec<BusinessOpeningHoursInterval>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ReactionType {
    #[serde(rename = "emoji")]
    Emoji { emoji: String },
    #[serde(rename = "custom_emoji")]
    CustomEmoji { custom_emoji_id: String },
    #[serde(rename = "paid")]
    Paid,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageOrigin {
    #[serde(rename = "user")]
    User { date: i64, sender_user: User },
    #[serde(rename = "hidden_user")]
    HiddenUser { date: i64, sender_user_name: String },
    #[serde(rename = "chat")]
    Chat {
        date: i64,
        sender_chat: Chat,
        #[serde(skip_serializing_if = "Option::is_none")]
        author_signature: Option<String>,
    },
    #[serde(rename = "channel")]
    Channel {
        date: i64,
        chat: Chat,
        message_id: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        author_signature: Option<String>,
    },
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LinkPreviewOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_small_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_large_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_above_text: Option<bool>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i64,
    pub height: i64,
    pub duration: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Audio {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Document {
    pub file_id: String,
    pub file_unique_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Video {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i64,
    pub height: i64,
    pub duration: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PaidMedia {
    #[serde(rename = "preview")]
    PaidMediaPreview {
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<i64>,
    },
    #[serde(rename = "photo")]
    PaidMediaPhoto { photo: Vec<PhotoSize> },
    #[serde(rename = "video")]
    PaidMediaVideo { video: Video },
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PaidMediaInfo {
    pub star_count: i64,
    pub paid_media: Vec<PaidMedia>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Story {
    pub chat: Chat,
    pub id: i64,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VideoNote {
    pub file_id: String,
    pub file_unique_id: String,
    pub length: i64,
    pub duration: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Voice {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Dice {
    pub emoji: String,
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageEntity {
    #[serde(rename = "mention")]
    Mention { offset: i64, length: i64 },
    #[serde(rename = "hashtag")]
    Hashtag { offset: i64, length: i64 },
    #[serde(rename = "cashtag")]
    Cashtag { offset: i64, length: i64 },
    #[serde(rename = "bot_command")]
    BotCommand { offset: i64, length: i64 },
    #[serde(rename = "url")]
    Url { offset: i64, length: i64 },
    #[serde(rename = "email")]
    Email { offset: i64, length: i64 },
    #[serde(rename = "phone_number")]
    PhoneNumber { offset: i64, length: i64 },
    #[serde(rename = "bold")]
    Bold { offset: i64, length: i64 },
    #[serde(rename = "italic")]
    Italic { offset: i64, length: i64 },
    #[serde(rename = "underline")]
    Underline { offset: i64, length: i64 },
    #[serde(rename = "strikethrough")]
    Strikethrough { offset: i64, length: i64 },
    #[serde(rename = "spoiler")]
    Spoiler { offset: i64, length: i64 },
    #[serde(rename = "blockquote")]
    Blockquote { offset: i64, length: i64 },
    #[serde(rename = "expandable_blockquote")]
    ExpandableBlockquote { offset: i64, length: i64 },
    #[serde(rename = "code")]
    Code { offset: i64, length: i64 },
    #[serde(rename = "pre")]
    Pre {
        offset: i64,
        length: i64,
        language: String,
    },
    #[serde(rename = "text_link")]
    TextLink {
        offset: i64,
        length: i64,
        url: String,
    },
    #[serde(rename = "text_mention")]
    TextMention {
        offset: i64,
        length: i64,
        user: User,
    },
    #[serde(rename = "custom_emoji")]
    CustomEmoji {
        offset: i64,
        length: i64,
        custom_emoji_id: String,
    },
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Giveaway {
    pub chats: Vec<Chat>,
    pub winners_selection_date: i64,
    pub winner_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_new_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_public_winners: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_codes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_month_count: Option<i64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GiveawayWinners {
    pub chat: Chat,
    pub giveaway_message_id: i64,
    pub winners_selection_date: i64,
    pub winner_count: i64,
    pub winners: Vec<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_chat_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_month_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_new_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_refunded: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_description: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub title: String,
    pub description: String,
    pub start_parameter: String,
    pub currency: String,
    pub total_amount: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PollOption {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
    pub voter_count: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Poll {
    pub id: String,
    pub question: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_entities: Option<Vec<MessageEntity>>,
    pub options: Vec<PollOption>,
    pub total_voter_count: i64,
    pub is_closed: bool,
    pub is_anonymous: bool,
    pub r#type: String,
    pub allows_multiple_answers: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Venue {
    pub location: Location,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalReplyInfo {
    pub origin: MessageOrigin,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat: Option<Chat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<PaidMediaInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<Story>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<VideoNote>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<Dice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Game>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway: Option<Giveaway>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_winners: Option<GiveawayWinners>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Invoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Venue>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ReplyParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_position: Option<i64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TextQuote {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    pub position: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_manual: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MessageAutoDeleteTimerChanged {
    pub message_auto_delete_time: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InaccessibleMessage {
    pub chat: Chat,
    pub message_id: i64,
    // 始终为0
    pub date: i64,
}

#[derive(Debug, Clone)]
pub enum MaybeInaccessibleMessage {
    // Message的date不为0
    Message(Box<Message>),
    // Message的date为0
    InaccessibleMessage(InaccessibleMessage),
}

impl Serialize for MaybeInaccessibleMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            MaybeInaccessibleMessage::Message(message) => message.serialize(serializer),
            MaybeInaccessibleMessage::InaccessibleMessage(inaccessible_message) => {
                inaccessible_message.serialize(serializer)
            }
        }
    }
}

impl<'de> Deserialize<'de> for MaybeInaccessibleMessage {
    fn deserialize<D>(deserializer: D) -> Result<MaybeInaccessibleMessage, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        if value
            .get("date")
            .map_or(false, |v| v.as_i64().unwrap_or(0) != 0)
        {
            Message::deserialize(value)
                .map(|m: Message| MaybeInaccessibleMessage::Message(Box::new(m)))
                .map_err(de::Error::custom)
        } else {
            InaccessibleMessage::deserialize(value)
                .map(MaybeInaccessibleMessage::InaccessibleMessage)
                .map_err(de::Error::custom)
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ShippingAddress {
    pub country_code: String,
    pub state: String,
    pub city: String,
    pub street_line1: String,
    pub street_line2: String,
    pub post_code: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OrderInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SuccessfulPayment {
    pub currency: String,
    pub total_amount: i64,
    pub invoice_payload: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RefundedPayment {
    pub currency: String,
    pub total_amount: i64,
    pub invoice_payload: String,
    pub telegram_payment_charge_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_payment_charge_id: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SharedUser {
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UsersShared {
    pub request_id: i64,
    pub users: Vec<SharedUser>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ChatShared {
    pub request_id: i64,
    pub chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WriteAccessAllowed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_attachment_menu: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PassportFile {
    pub file_id: String,
    pub file_unique_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_date: Option<i64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EncryptedPassportElement {
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<PassportFile>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_side: Option<PassportFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_side: Option<PassportFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<PassportFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<PassportFile>>,
    pub hash: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EncryptedCredentials {
    pub data: String,
    pub hash: String,
    pub secret: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProximityAlertTriggered {
    pub traveler: User,
    pub watcher: User,
    pub distance: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ChatBoostAdded {
    pub boost_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BackgroundFill {
    #[serde(rename = "solid")]
    Solid { color: i64 },
    #[serde(rename = "gradient")]
    Gradient {
        top_color: i64,
        bottom_color: i64,
        rotation_angle: i64,
    },
    #[serde(rename = "freeform_gradient")]
    FreeformGradient { colors: Vec<i64> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BackgroundType {
    #[serde(rename = "fill")]
    Fill { fill: BackgroundFill },
    #[serde(rename = "wallpaper")]
    Wallpaper {
        document: Document,
        dark_theme_dimming: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_blurred: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_moving: Option<bool>,
    },
    #[serde(rename = "pattern")]
    Pattern {
        document: Document,
        fill: BackgroundFill,
        intensity: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_inverted: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_moving: Option<bool>,
    },
    #[serde(rename = "chat_theme")]
    ChatTheme { theme_name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBackground {
    pub r#type: BackgroundType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ForumTopicCreated {
    pub name: String,
    pub icon_color: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ForumTopicEdited {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ForumTopicClosed;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ForumTopicReopened;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GeneralForumTopicHidden;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GeneralForumTopicUnhidden;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GiveawayCreated;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GiveawayCompleted {
    pub winner_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_message: Option<Box<Message>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VideoChatScheduled {
    pub start_date: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VideoChatStarted;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VideoChatEnded {
    pub duration: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VideoChatParticipantsInvited {
    pub users: Vec<User>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WebAppData {
    pub data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_text: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]

pub struct LoginUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_write_access: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SwitchInlineQueryChosenChat {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_bot_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_group_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_channel_chats: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CallbackGame;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InlineKeyboardButton {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_url: Option<LoginUrl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_game: Option<CallbackGame>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Message {
    pub message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_chat: Option<Chat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_boost_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_business_bot: Option<User>,
    pub date: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat: Chat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_origin: Option<MessageOrigin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_topic_message: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_forward: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message: Option<Box<Message>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reply: Option<ExternalReplyInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<TextQuote>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_story: Option<Story>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_bot: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_from_offline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<PaidMediaInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<Story>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<VideoNote>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<Dice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Game>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Venue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_members: Option<Vec<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_chat_member: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_chat_photo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_chat_created: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supergroup_chat_created: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_chat_created: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_from_chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<MaybeInaccessibleMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Invoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_payment: Option<SuccessfulPayment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_payment: Option<RefundedPayment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users_shared: Option<UsersShared>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_shared: Option<ChatShared>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_access_allowed: Option<WriteAccessAllowed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport_data: Option<PassportData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_added: Option<ChatBoostAdded>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_background_set: Option<ChatBackground>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_created: Option<ForumTopicCreated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_edited: Option<ForumTopicEdited>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_closed: Option<ForumTopicClosed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_reopened: Option<ForumTopicReopened>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_forum_topic_hidden: Option<GeneralForumTopicHidden>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_forum_topic_unhidden: Option<GeneralForumTopicUnhidden>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_created: Option<GiveawayCreated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway: Option<Giveaway>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_winners: Option<GiveawayWinners>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_completed: Option<GiveawayCompleted>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_scheduled: Option<VideoChatScheduled>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_started: Option<VideoChatStarted>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_ended: Option<VideoChatEnded>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_participants_invited: Option<VideoChatParticipantsInvited>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_data: Option<WebAppData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MessageId {
    pub message_id: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InputPollOption {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PollAnswer {
    pub poll_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voter_chat: Option<Chat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    pub option_ids: Vec<i64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UserProfilePhotos {
    pub total_count: i64,
    pub photos: Vec<Vec<PhotoSize>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WebAppInfo {
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_persistent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_keyboard: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_keyboard: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct KeyboardButton {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_users: Option<KeyboardButtonRequestUsers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_chat: Option<KeyboardButtonRequestChat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_poll: Option<KeyboardButtonPollType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct KeyboardButtonRequestUsers {
    pub request_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_premium: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_quantity: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_username: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_photo: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct KeyboardButtonRequestChat {
    pub request_id: i64,
    pub chat_is_channel: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_forum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_has_username: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_created: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_administrator_rights: Option<ChatAdministratorRights>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_administrator_rights: Option<ChatAdministratorRights>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_is_member: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_title: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_username: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_photo: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ChatAdministratorRights {
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
pub struct KeyboardButtonPollType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ReplyKeyboardRemove {
    pub remove_keyboard: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: MaybeInaccessibleMessage,
    pub inline_message_id: String,
    pub chat_instance: String,
    pub data: String,
    pub game_short_name: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ForceReply {
    pub force_reply: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ChatInviteLink {
    pub invite_link: String,
    pub creator: User,
    pub creates_join_request: bool,
    pub is_primary: bool,
    pub is_revoked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_join_request_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberUpdated {
    pub chat: Chat,
    pub from: User,
    pub date: i64,
    pub old_chat_member: ChatMember,
    pub new_chat_member: ChatMember,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_join_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_chat_folder_invite_link: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum ChatMember {
    #[serde(rename = "creator")]
    Owner {
        user: User,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_anonymous: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        custom_title: Option<String>,
    },
    #[serde(rename = "administrator")]
    Administrator {
        user: User,
        can_be_edited: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_anonymous: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        can_manage_chat: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        can_delete_messages: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        can_manage_video_chats: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        can_restrict_members: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        can_promote_members: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        can_change_info: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        can_invite_users: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        can_post_stories: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        can_edit_stories: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        can_delete_stories: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        can_post_messages: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        can_edit_messages: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        can_pin_messages: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        can_manage_topics: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        custom_title: Option<String>,
    },
    #[serde(rename = "member")]
    Member {
        user: User,
        #[serde(skip_serializing_if = "Option::is_none")]
        until_date: Option<i64>,
    },
    #[serde(rename = "restricted")]
    Restricted {
        user: User,
        is_member: bool,
        can_send_messages: bool,
        can_send_audios: bool,
        can_send_documents: bool,
        can_send_photos: bool,
        can_send_videos: bool,
        can_send_video_notes: bool,
        can_send_voice_notes: bool,
        can_send_polls: bool,
        can_send_other_messages: bool,
        can_add_web_page_previews: bool,
        can_change_info: bool,
        can_invite_users: bool,
        can_pin_messages: bool,
        can_manage_topics: bool,
        until_date: i64,
    },
    #[serde(rename = "left")]
    Left { user: User },
    #[serde(rename = "kicked")]
    Banned {
        user: User,
        #[serde(skip_serializing_if = "Option::is_none")]
        until_date: Option<i64>,
    },
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ChatJoinRequest {
    pub chat: Chat,
    pub from: User,
    pub user_chat_id: i64,
    pub date: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ChatPermissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_audios: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_documents: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_photos: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_videos: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_video_notes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_voice_notes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_polls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ChatLocation {
    pub location: Location,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionCount {
    pub r#type: ReactionType,
    pub total_count: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MessageReactionUpdated {
    pub chat: Chat,
    pub message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_chat: Option<Chat>,
    pub date: i64,
    pub old_reaction: Vec<ReactionType>,
    pub new_reaction: Vec<ReactionType>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MessageReactionCountUpdated {
    pub chat: Chat,
    pub message_id: i64,
    pub date: i64,
    pub reactions: Vec<ReactionCount>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ForumTopic {
    pub message_thread_id: i64,
    pub name: String,
    pub icon_color: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BotCommand {
    pub command: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BotCommandScope {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "all_private_chats")]
    AllPrivateChats,
    #[serde(rename = "all_group_chats")]
    AllGroupChats,
    #[serde(rename = "all_chat_administrators")]
    AllChatAdministrators,
    #[serde(rename = "chat")]
    Chat { chat_id: String },
    #[serde(rename = "chat_administrators")]
    ChatAdministrators { chat_id: String },
    #[serde(rename = "chat_member")]
    ChatMember { chat_id: String, user_id: i64 },
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BotName {
    pub name: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BotDescription {
    pub description: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BotShortDescription {
    pub short_description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MenuButton {
    #[serde(rename = "commands")]
    Commands,
    #[serde(rename = "web_app")]
    WebApp { text: String, web_app: WebAppInfo },
    #[serde(rename = "default")]
    Default,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "source")]
pub enum ChatBoostSource {
    #[serde(rename = "premium")]
    Premium { user: User },
    #[serde(rename = "gift_code")]
    GiftCode { user: User },
    #[serde(rename = "giveaway")]
    Giveaway {
        giveaway_message_id: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        user: Option<User>,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_unclaimed: Option<bool>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoost {
    pub boost_id: String,
    pub add_date: i64,
    pub expiration_date: i64,
    pub source: ChatBoostSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoostUpdated {
    pub chat: Chat,
    pub boost: ChatBoost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoostRemoved {
    pub chat: Chat,
    pub boost_id: String,
    pub remove_date: i64,
    pub source: ChatBoostSource,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UserChatBoosts {
    pub boosts: Vec<ChatBoost>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BusinessConnection {
    pub id: String,
    pub user: User,
    pub user_chat_id: i64,
    pub date: i64,
    pub can_reply: bool,
    pub is_enabled: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BusinessMessagesDeleted {
    pub business_connection_id: String,
    pub chat: Chat,
    pub message_ids: Vec<i64>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ResponseParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InputMedia {
    #[serde(rename = "photo")]
    Photo {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        show_caption_above_media: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        has_spoiler: Option<bool>,
    },
    #[serde(rename = "video")]
    Video {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        show_caption_above_media: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        supports_streaming: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        has_spoiler: Option<bool>,
    },
    #[serde(rename = "animation")]
    Animation {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        show_caption_above_media: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        has_spoiler: Option<bool>,
    },
    #[serde(rename = "audio")]
    Audio {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        performer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },

    #[serde(rename = "document")]
    Document {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption_entities: Option<Vec<MessageEntity>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        disable_content_type_detection: Option<bool>,
    },
}

impl InputMedia {
    pub(crate) async fn try_part(
        &mut self,
        mut form: reqwest::multipart::Form,
    ) -> anyhow::Result<reqwest::multipart::Form> {
        match self {
            Self::Photo { media, .. } => {
                let media_clone = media.clone();
                let path = std::path::Path::new(&media_clone);
                if path.is_file() {
                    let filename = path
                        .file_name()
                        .and_then(|f| f.to_str())
                        .and_then(|f| Some(f.to_string()))
                        .unwrap_or_else(|| {
                            // 生成一个16位的随机字符串
                            let mut rng = rand::thread_rng();
                            let mut filename = Vec::with_capacity(16);
                            for _ in 0..16 {
                                filename.push(rng.gen_range(b'a'..=b'z') as char);
                            }
                            filename.into_iter().collect::<String>()
                        });
                    *media = format!("attach://{}", filename);
                    form = file_to_multipart(filename, path, form).await?;
                }

                Ok(form)
            }
            InputMedia::Video {
                media, thumbnail, ..
            }
            | InputMedia::Animation {
                media, thumbnail, ..
            }
            | InputMedia::Audio {
                media, thumbnail, ..
            }
            | InputMedia::Document {
                media, thumbnail, ..
            } => {
                let media_clone = media.clone();
                let path = std::path::Path::new(&media_clone);
                if path.is_file() {
                    let filename = path
                        .file_name()
                        .and_then(|f| f.to_str())
                        .and_then(|f| Some(f.to_string()))
                        .unwrap_or_else(|| {
                            // 生成一个16位的随机字符串
                            let mut rng = rand::thread_rng();
                            let mut filename = Vec::with_capacity(16);
                            for _ in 0..16 {
                                filename.push(rng.gen_range(b'a'..=b'z') as char);
                            }
                            filename.into_iter().collect::<String>()
                        });
                    *media = format!("attach://{}", filename);
                    form = file_to_multipart(filename, path, form).await?;
                }
                if let Some(thumbnail) = thumbnail {
                    let thumbnail_clone = thumbnail.clone();
                    let path = std::path::Path::new(&thumbnail_clone);
                    if path.is_file() {
                        let filename = path
                            .file_name()
                            .and_then(|f| f.to_str())
                            .and_then(|f| Some(f.to_string()))
                            .unwrap_or_else(|| {
                                // 生成一个16位的随机字符串
                                let mut rng = rand::thread_rng();
                                let mut filename = Vec::with_capacity(16);
                                for _ in 0..16 {
                                    filename.push(rng.gen_range(b'a'..=b'z') as char);
                                }
                                filename.into_iter().collect::<String>()
                            });
                        *thumbnail = format!("attach://{}", filename);
                        form = file_to_multipart(filename, path, form).await?;
                    }
                }
                Ok(form)
            }
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InputPaidMedia {
    #[serde(rename = "photo")]
    Photo { media: String },
    #[serde(rename = "video")]
    Video {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        supports_streaming: Option<bool>,
    },
}

impl InputPaidMedia {
    pub(crate) async fn try_part(
        &mut self,
        mut form: reqwest::multipart::Form,
    ) -> anyhow::Result<reqwest::multipart::Form> {
        match self {
            InputPaidMedia::Photo { media } => {
                let path = Path::new(media);
                if path.exists() {
                    let filename = path
                        .file_name()
                        .and_then(|f| f.to_str())
                        .and_then(|f| Some(f.to_string()))
                        .unwrap_or_else(|| {
                            // 生成一个16位的随机字符串
                            let mut rng = rand::thread_rng();
                            let mut filename = Vec::with_capacity(16);
                            for _ in 0..16 {
                                filename.push(rng.gen_range(b'a'..=b'z') as char);
                            }
                            filename.into_iter().collect::<String>()
                        });
                    form = file_to_multipart(filename.clone(), path, form).await?;
                    *media = format!("attach://{}", filename);
                }
                Ok(form)
            }
            InputPaidMedia::Video {
                media, thumbnail, ..
            } => {
                let media_clone = media.clone();
                let path = std::path::Path::new(&media_clone);
                if path.is_file() {
                    let filename = path
                        .file_name()
                        .and_then(|f| f.to_str())
                        .and_then(|f| Some(f.to_string()))
                        .unwrap_or_else(|| {
                            // 生成一个16位的随机字符串
                            let mut rng = rand::thread_rng();
                            let mut filename = Vec::with_capacity(16);
                            for _ in 0..16 {
                                filename.push(rng.gen_range(b'a'..=b'z') as char);
                            }
                            filename.into_iter().collect::<String>()
                        });
                    *media = format!("attach://{}", filename);
                    form = file_to_multipart(filename, path, form).await?;
                }
                if let Some(thumbnail) = thumbnail {
                    let thumbnail_clone = thumbnail.clone();
                    let path = std::path::Path::new(&thumbnail_clone);
                    if path.is_file() {
                        let filename = path
                            .file_name()
                            .and_then(|f| f.to_str())
                            .and_then(|f| Some(f.to_string()))
                            .unwrap_or_else(|| {
                                // 生成一个16位的随机字符串
                                let mut rng = rand::thread_rng();
                                let mut filename = Vec::with_capacity(16);
                                for _ in 0..16 {
                                    filename.push(rng.gen_range(b'a'..=b'z') as char);
                                }
                                filename.into_iter().collect::<String>()
                            });
                        *thumbnail = format!("attach://{}", filename);
                        form = file_to_multipart(filename, path, form).await?;
                    }
                }
                Ok(form)
            }
        }
    }
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BusinessIntro {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ChatFullInfo {
    pub id: i64,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_reaction_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_usernames: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<Birthdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_intro: Option<BusinessIntro>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_location: Option<BusinessLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_opening_hours: Option<BusinessOpeningHours>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_chat: Option<Chat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_reactions: Option<Vec<ReactionType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_custom_emoji_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_accent_color_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_background_custom_emoji_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_custom_emoji_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_expiration_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_private_forwards: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_restricted_voice_and_video_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_to_send_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_by_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ChatPermissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_paid_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_mode_delay: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unrestrict_boost_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_aggressive_anti_spam_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_hidden_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_visible_history: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_set_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_set_sticker_set: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_sticker_set_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ChatLocation>,
}
