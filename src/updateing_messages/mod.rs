pub mod payload;

use crate::{
    available_types::{Message, Poll},
    bot::Bot,
};
use anyhow::Result;
use payload::*;

impl Bot {
    pub async fn edit_message_text(&self, payload: &EditMessageTextPayload) -> Result<Message> {
        self.call_api_json("editMessageTest", payload).await
    }

    pub async fn edit_message_caption(
        &self,
        payload: &EditMessageCaptionPayload,
    ) -> Result<Message> {
        self.call_api_json("editMessageCaption", payload).await
    }

    pub async fn edit_message_media(&self, payload: &EditMessageMediaPayload) -> Result<Message> {
        self.call_api_json("editMessageMedia", payload).await
    }

    pub async fn edit_message_live_location(
        &self,
        payload: &EditMessageLiveLocationPayload,
    ) -> Result<Message> {
        self.call_api_json("editMessageLiveLocation", payload).await
    }

    pub async fn stop_message_live_location(
        &self,
        payload: &StopMessageLiveLocationPayload,
    ) -> Result<Message> {
        self.call_api_json("stopMessageLiveLocation", payload).await
    }

    pub async fn edit_message_reply_markup(
        &self,
        payload: &EditMessageReplyMarkupPayload,
    ) -> Result<Message> {
        self.call_api_json("editMessageReplyMarkup", payload).await
    }

    pub async fn stop_poll(&self, payload: &StopPollPayload) -> Result<Poll> {
        self.call_api_json("stopPoll", payload).await
    }

    pub async fn delete_message(&self, payload: &DeleteMessagePayload) -> Result<bool> {
        self.call_api_json("deleteMessage", payload).await
    }

    pub async fn delete_messages(&self, payload: &DeleteMessagesPayload) -> Result<bool> {
        self.call_api_json("deleteMessages", payload).await
    }
}
