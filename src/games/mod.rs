use payload::*;
use serde_json::Value;
use types::GameHighScore;

use crate::{available_types::Message, bot::Bot};
use anyhow::Result;
pub mod payload;
pub mod types;

impl Bot {
    pub async fn send_game(&self, payload: &SendGamePayload) -> Result<Message> {
        self.call_api_json("sendGame", payload).await
    }

    pub async fn callback_game(&self) -> Result<Value> {
        self.call_api_no_payload("callbackGame").await
    }

    pub async fn set_game_score(&self, payload: &SetGameScorePayload) -> Result<Message> {
        self.call_api_json("setGameScore", payload).await
    }

    pub async fn get_game_high_scores(
        &self,
        payload: &GetGameHighScoresPayload,
    ) -> Result<Vec<GameHighScore>> {
        self.call_api_json("getGameHighScores", payload).await
    }
}
