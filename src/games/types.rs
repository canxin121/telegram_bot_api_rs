use serde::{Deserialize, Serialize};

use crate::available_types::{Animation, MessageEntity, PhotoSize, User};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameHighScore {
    pub position: i64,
    pub user: User,
    pub score: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
