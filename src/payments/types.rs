use serde::{Deserialize, Serialize};

use crate::available_types::{PaidMedia, User};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub title: String,
    pub description: String,
    pub start_parameter: String,
    pub currency: String,
    pub total_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingAddress {
    pub country_code: String,
    pub state: String,
    pub city: String,
    pub street_line1: String,
    pub street_line2: String,
    pub post_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingOption {
    pub id: String,
    pub title: String,
    pub prices: Vec<LabeledPrice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundedPayment {
    pub currency: String,
    pub total_amount: i64,
    pub invoice_payload: String,
    pub telegram_payment_charge_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_payment_charge_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingQuery {
    pub id: String,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: ShippingAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreCheckoutQuery {
    pub id: String,
    pub from: User,
    pub currency: String,
    pub total_amount: i64,
    pub invoice_payload: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RevenueWithdrawalState {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "succeeded")]
    Succeeded { date: i64, url: String },
    #[serde(rename = "failed")]
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TransactionPartner {
    #[serde(rename = "user")]
    User {
        user: User,
        #[serde(skip_serializing_if = "Option::is_none")]
        invoice_payload: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        paid_media: Option<Vec<PaidMedia>>,
    },
    #[serde(rename = "fragment")]
    Fragment {
        #[serde(skip_serializing_if = "Option::is_none")]
        withdrawal_state: Option<RevenueWithdrawalState>,
    },
    #[serde(rename = "telegram_ads")]
    TelegramAds,
    #[serde(rename = "other")]
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarTransaction {
    pub id: String,
    pub amount: i64,
    pub date: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<TransactionPartner>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<TransactionPartner>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarTransactions {
    pub transactions: Vec<StarTransaction>,
}
