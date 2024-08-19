use serde::{Deserialize, Serialize};

use crate::{
    available_types::{
        BusinessConnection, BusinessMessagesDeleted, CallbackQuery, ChatBoostRemoved,
        ChatBoostUpdated, ChatJoinRequest, ChatMemberUpdated, Message, MessageReactionCountUpdated,
        MessageReactionUpdated, Poll, PollAnswer,
    },
    inline_mode::types::{ChosenInlineResult, InlineQuery},
    payments::types::{PreCheckoutQuery, ShippingQuery},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Update {
    pub update_id: i64,
    #[serde(flatten)]
    pub data: UpdateData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateData {
    Message {
        message: Message,
    },
    EditedMessage {
        edited_message: Message,
    },
    ChannelPost {
        channel_post: Message,
    },
    EditedChannelPost {
        edited_channel_post: Message,
    },
    BusinessConnection {
        business_connection: BusinessConnection,
    },
    BusinessMessage {
        business_message: Message,
    },
    EditedBusinessMessage {
        edited_business_message: Message,
    },
    DeletedBusinessMessages {
        deleted_business_messages: BusinessMessagesDeleted,
    },
    MessageReaction {
        message_reaction: MessageReactionUpdated,
    },
    MessageReactionCount {
        message_reaction_count: MessageReactionCountUpdated,
    },
    InlineQuery {
        inline_query: InlineQuery,
    },
    ChosenInlineResult {
        chosen_inline_result: ChosenInlineResult,
    },
    CallbackQuery {
        callback_query: CallbackQuery,
    },
    ShippingQuery {
        shipping_query: ShippingQuery,
    },
    PreCheckoutQuery {
        pre_checkout_query: PreCheckoutQuery,
    },
    Poll {
        poll: Poll,
    },
    PollAnswer {
        poll_answer: PollAnswer,
    },
    MyChatMember {
        my_chat_member: ChatMemberUpdated,
    },
    ChatMember {
        chat_member: ChatMemberUpdated,
    },
    ChatJoinRequest {
        chat_join_request: ChatJoinRequest,
    },
    ChatBoost {
        chat_boost: ChatBoostUpdated,
    },
    RemovedChatBoost {
        removed_chat_boost: ChatBoostRemoved,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllowedUpdateType {
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "edited_message")]
    EditedMessage,
    #[serde(rename = "channel_post")]
    ChannelPost,
    #[serde(rename = "edited_channel_post")]
    EditedChannelPost,
    #[serde(rename = "business_connection")]
    BusinessConnection,
    #[serde(rename = "business_message")]
    BusinessMessage,
    #[serde(rename = "edited_business_message")]
    EditedBusinessMessage,
    #[serde(rename = "deleted_business_messages")]
    DeletedBusinessMessages,
    #[serde(rename = "message_reaction")]
    MessageReaction,
    #[serde(rename = "message_reaction_count")]
    MessageReactionCount,
    #[serde(rename = "inline_query")]
    InlineQuery,
    #[serde(rename = "chosen_inline_result")]
    ChosenInlineResult,
    #[serde(rename = "callback_query")]
    CallbackQuery,
    #[serde(rename = "shipping_query")]
    ShippingQuery,
    #[serde(rename = "pre_checkout_query")]
    PreCheckoutQuery,
    #[serde(rename = "poll")]
    Poll,
    #[serde(rename = "poll_answer")]
    PollAnswer,
    #[serde(rename = "my_chat_member")]
    MyChatMember,
    #[serde(rename = "chat_member")]
    ChatMember,
    #[serde(rename = "chat_join_request")]
    ChatJoinRequest,
    #[serde(rename = "chat_boost")]
    ChatBoost,
    #[serde(rename = "removed_chat_boost")]
    RemovedChatBoost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookInfo {
    pub url: String,
    pub has_custom_certificate: bool,
    pub pending_update_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_synchronization_error_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}
