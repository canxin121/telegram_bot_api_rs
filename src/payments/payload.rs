use serde::{Deserialize, Serialize};

use crate::available_types::{InlineKeyboardMarkup, ReplyParameters};

use super::types::{LabeledPrice, ShippingOption};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendInvoicePayload {
    pub chat_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub title: String,
    pub description: String,
    pub payload: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_token: Option<String>,
    pub currency: String,
    pub prices: Vec<LabeledPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
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
pub struct CreateInvoiceLinkPayload {
    pub title: String,
    pub description: String,
    pub payload: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_token: Option<String>,
    pub currency: String,
    pub prices: Vec<LabeledPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerShippingQueryPayload {
    pub shipping_query_id: String,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<Vec<ShippingOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerPreCheckoutQueryPayload {
    pub pre_checkout_query_id: String,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarTransaction {
    pub id: String,
    pub from_user_id: i64,
    pub to_user_id: i64,
    pub amount: i64,
    pub currency: String,
    pub status: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundStarPaymentPayload {
    pub user_id: i64,
    pub telegram_payment_charge_id: String,
}
