use crate::bot::Bot;
use anyhow::Result;
use resp::SentWebAppMessage;
pub mod payload;
pub mod resp;
pub mod types;

impl Bot {
    pub async fn answer_web_app_query(
        &self,
        payload: &payload::AnswerWebAppQuery,
    ) -> Result<SentWebAppMessage> {
        self.call_api_json("answerWebAppQuery", payload).await
    }
}
