use serde::{Deserialize, Serialize};

use super::types::InlineQueryResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerWebAppQuery {
    pub web_app_query_id: String,
    pub result: InlineQueryResult,
}
