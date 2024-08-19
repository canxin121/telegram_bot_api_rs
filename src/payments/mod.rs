use anyhow::Result;
use payload::*;
use types::StarTransactions;

use crate::{available_types::Message, bot::Bot};

pub mod payload;
pub mod types;

impl Bot {
    pub async fn send_invoice(&self, payload: &SendInvoicePayload) -> Result<Message> {
        self.call_api_json("sendInvoice", payload).await
    }
    pub async fn create_invoice_link(&self, payload: &CreateInvoiceLinkPayload) -> Result<String> {
        self.call_api_json("createInvoiceLink", payload).await
    }

    pub async fn answer_shipping_query(
        &self,
        payload: &AnswerShippingQueryPayload,
    ) -> Result<bool> {
        self.call_api_json("answerShippingQuery", payload).await
    }

    pub async fn answer_pre_checkout_query(
        &self,
        payload: &AnswerPreCheckoutQueryPayload,
    ) -> Result<bool> {
        self.call_api_json("answerPreCheckoutQuery", payload).await
    }

    pub async fn get_star_transactions(&self) -> Result<StarTransactions> {
        self.call_api_json("getStarTransactions", &serde_json::json!({}))
            .await
    }

    pub async fn refund_star_payment(&self, payload: &RefundStarPaymentPayload) -> Result<bool> {
        self.call_api_json("refundStarPayment", payload).await
    }
}
