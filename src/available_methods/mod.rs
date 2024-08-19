pub mod payload;

use std::path::Path;

use anyhow::Result;
use payload::*;

use crate::{
    available_types::{
        BotCommand, BotDescription, BotName, BotShortDescription, BusinessConnection,
        ChatAdministratorRights, ChatFullInfo, ChatInviteLink, ChatMember, File, ForumTopic,
        MenuButton, Message, MessageId, User, UserChatBoosts, UserProfilePhotos,
    },
    bot::Bot,
    stickers::types::Sticker,
};

impl Bot {
    pub async fn get_me(&self) -> Result<User> {
        self.call_api_no_payload("getMe").await
    }

    pub async fn log_out(&self) -> Result<()> {
        self.call_api_no_payload("logOut").await
    }

    pub async fn send_message(&self, payload: &SendMessagePayload) -> Result<Message> {
        self.call_api_json("sendMessage", payload).await
    }
    pub async fn forward_message(&self, payload: &ForwardMessagePayload) -> Result<Message> {
        self.call_api_json("forwardMessage", payload).await
    }

    pub async fn forward_messages(
        &self,
        payload: &ForwardMessagesPayload,
    ) -> Result<Vec<MessageId>> {
        self.call_api_json("forwardMessages", payload).await
    }

    pub async fn copy_message(&self, payload: &CopyMessagePayload) -> Result<MessageId> {
        self.call_api_json("copyMessage", payload).await
    }

    pub async fn copy_messages(&self, payload: &CopyMessagesPayload) -> Result<Vec<MessageId>> {
        self.call_api_json("copyMessages", payload).await
    }

    pub async fn send_photo(&self, payload: SendPhotoPayload) -> Result<Message> {
        self.call_api_multipart("sendPhoto", payload).await
    }

    pub async fn send_audio(&self, payload: SendAudioPayload) -> Result<Message> {
        self.call_api_multipart("sendAudio", payload).await
    }

    pub async fn send_document(&self, payload: SendDocumentPayload) -> Result<Message> {
        self.call_api_multipart("sendDocument", payload).await
    }

    pub async fn send_video(&self, payload: SendVideoPayload) -> Result<Message> {
        self.call_api_multipart("sendVideo", payload).await
    }

    pub async fn send_animation(&self, payload: SendAnimationPayload) -> Result<Message> {
        self.call_api_multipart("sendAnimation", payload).await
    }

    pub async fn send_voice(&self, payload: SendVoicePayload) -> Result<Message> {
        self.call_api_multipart("sendVoice", payload).await
    }

    pub async fn send_video_note(&self, payload: SendVideoNotePayload) -> Result<Message> {
        self.call_api_multipart("sendVideoNote", payload).await
    }

    pub async fn send_paid_media(&self, payload: SendPaidMediaPayload) -> Result<Message> {
        self.call_api_multipart("sendPaidMedia", payload).await
    }

    pub async fn send_media_group(&self, payload: SendMediaGroupPayload) -> Result<Vec<Message>> {
        self.call_api_multipart("sendMediaGroup", payload).await
    }

    pub async fn send_location(&self, payload: &SendLocationPayload) -> Result<Message> {
        self.call_api_json("sendLocation", payload).await
    }

    pub async fn send_venue(&self, payload: &SendVenuePayload) -> Result<Message> {
        self.call_api_json("sendVenue", payload).await
    }

    pub async fn send_contact(&self, payload: &SendContactPayload) -> Result<Message> {
        self.call_api_json("sendContact", payload).await
    }

    pub async fn send_poll(&self, payload: &SendPollPayload) -> Result<Message> {
        self.call_api_json("sendPoll", payload).await
    }

    pub async fn send_dice(&self, payload: &SendDicePayload) -> Result<Message> {
        self.call_api_json("sendDice", payload).await
    }

    pub async fn send_chat_action(&self, payload: &SendChatActionPayload) -> Result<bool> {
        self.call_api_json("sendChatAction", payload).await
    }

    pub async fn set_message_reaction(&self, payload: &SetMessageReactionPayload) -> Result<bool> {
        self.call_api_json("setMessageReaction", payload).await
    }

    pub async fn get_user_profile_photos(
        &self,
        payload: &GetUserProfilePhotosPayload,
    ) -> Result<UserProfilePhotos> {
        self.call_api_json("getUserProfilePhotos", payload).await
    }

    pub async fn get_file(&self, payload: &GetFilePayload) -> Result<File> {
        self.call_api_json("getFile", payload).await
    }

    pub async fn ban_chat_member(&self, payload: &BanChatMemberPayload) -> Result<bool> {
        self.call_api_json("banChatMember", payload).await
    }

    pub async fn unban_chat_member(&self, payload: &UnbanChatMemberPayload) -> Result<bool> {
        self.call_api_json("unbanChatMember", payload).await
    }

    pub async fn restrict_chat_member(&self, payload: &RestrictChatMemberPayload) -> Result<bool> {
        self.call_api_json("restrictChatMember", payload).await
    }

    pub async fn promote_chat_member(&self, payload: &PromoteChatMemberPayload) -> Result<bool> {
        self.call_api_json("promoteChatMember", payload).await
    }

    pub async fn set_chat_administrator_custom_title(
        &self,
        payload: &SetChatAdministratorCustomTitlePayload,
    ) -> Result<bool> {
        self.call_api_json("setChatAdministratorCustomTitle", payload)
            .await
    }

    pub async fn ban_chat_sender_chat(&self, payload: &BanChatMemberPayload) -> Result<bool> {
        self.call_api_json("banChatSenderChat", payload).await
    }

    pub async fn unban_chat_sender_chat(&self, payload: &UnbanChatMemberPayload) -> Result<bool> {
        self.call_api_json("unbanChatSenderChat", payload).await
    }

    pub async fn set_chat_permissions(&self, payload: &SetChatPermissionsPayload) -> Result<bool> {
        self.call_api_json("setChatPermissions", payload).await
    }

    pub async fn export_chat_invite_link(
        &self,
        payload: &ExportChatInviteLinkPayload,
    ) -> Result<String> {
        self.call_api_json("exportChatInviteLink", payload).await
    }

    pub async fn create_chat_invite_link(
        &self,
        payload: &CreateChatInviteLinkPayload,
    ) -> Result<ChatInviteLink> {
        self.call_api_json("createChatInviteLink", payload).await
    }

    pub async fn edit_chat_invite_link(
        &self,
        payload: &EditChatInviteLinkPayload,
    ) -> Result<ChatInviteLink> {
        self.call_api_json("editChatInviteLink", payload).await
    }

    pub async fn create_chat_subscription_invite_link(
        &self,
        payload: &CreateChatSubscriptionInviteLinkPayload,
    ) -> Result<ChatInviteLink> {
        self.call_api_json("createChatSubscriptionInviteLink", payload)
            .await
    }

    pub async fn edit_chat_subscription_invite_link(
        &self,
        payload: &EditChatSubscriptionInviteLinkPayload,
    ) -> Result<ChatInviteLink> {
        self.call_api_json("editChatSubscriptionInviteLink", payload)
            .await
    }

    pub async fn revoke_chat_invite_link(
        &self,
        payload: &RevokeChatInviteLinkPayload,
    ) -> Result<ChatInviteLink> {
        self.call_api_json("revokeChatInviteLink", payload).await
    }

    pub async fn approve_chat_join_request(
        &self,
        payload: &ApproveChatJoinRequestPayload,
    ) -> Result<bool> {
        self.call_api_json("approveChatJoinRequest", payload).await
    }

    pub async fn decline_chat_join_request(
        &self,
        payload: &DeclineChatJoinRequestPayload,
    ) -> Result<bool> {
        self.call_api_json("declineChatJoinRequest", payload).await
    }

    pub async fn set_chat_photo<P: AsRef<Path>>(
        &self,
        payload: SetChatPhotoPayload,
    ) -> Result<bool> {
        self.call_api_multipart("setChatPhoto", payload).await
    }

    pub async fn delete_chat_photo(&self, payload: &DeleteChatPhotoPayload) -> Result<bool> {
        self.call_api_json("deleteChatPhoto", payload).await
    }

    pub async fn set_chat_title(&self, payload: &SetChatTitlePayload) -> Result<bool> {
        self.call_api_json("setChatTitle", payload).await
    }

    pub async fn set_chat_description(&self, payload: &SetChatDescriptionPayload) -> Result<bool> {
        self.call_api_json("setChatDescription", payload).await
    }

    pub async fn pin_chat_message(&self, payload: &PinChatMessagePayload) -> Result<bool> {
        self.call_api_json("pinChatMessage", payload).await
    }

    pub async fn unpin_chat_message(&self, payload: &UnpinChatMessagePayload) -> Result<bool> {
        self.call_api_json("unpinChatMessage", payload).await
    }

    pub async fn unpin_all_chat_messages(&self, payload: &UnpinChatMessagePayload) -> Result<bool> {
        self.call_api_json("unpinAllChatMessages", payload).await
    }

    pub async fn leave_chat(&self, payload: &ChatIdPayload) -> Result<bool> {
        self.call_api_json("leaveChat", payload).await
    }

    pub async fn get_chat(&self, payload: &ChatIdPayload) -> Result<ChatFullInfo> {
        self.call_api_json("getChat", payload).await
    }

    pub async fn get_chat_administrators(
        &self,
        payload: &ChatIdPayload,
    ) -> Result<Vec<ChatMember>> {
        self.call_api_json("getChatAdministrators", payload).await
    }

    pub async fn get_chat_member_count(&self, payload: &ChatIdPayload) -> Result<i64> {
        self.call_api_json("getChatMemberCount", payload).await
    }

    pub async fn get_chat_member(&self, payload: &GetChatMemberPayload) -> Result<ChatMember> {
        self.call_api_json("getChatMember", payload).await
    }

    pub async fn set_chat_sticker_set(&self, payload: &SetChatStickerSetPayload) -> Result<bool> {
        self.call_api_json("setChatStickerSet", payload).await
    }

    pub async fn delete_chat_sticker_set(
        &self,
        payload: &DeleteChatStickerSetPayload,
    ) -> Result<bool> {
        self.call_api_json("deleteChatStickerSet", payload).await
    }

    pub async fn get_forum_topic_icon_stickers(&self) -> Result<Vec<Sticker>> {
        self.call_api_no_payload("getForumTopicIconStickers").await
    }

    pub async fn create_forum_topic(
        &self,
        payload: &CreateForumTopicPayload,
    ) -> Result<ForumTopic> {
        self.call_api_json("createForumTopic", payload).await
    }

    pub async fn edit_forum_topic(&self, payload: &EditForumTopicPayload) -> Result<bool> {
        self.call_api_json("editForumTopic", payload).await
    }

    pub async fn close_forum_topic(&self, payload: &ForumTopicPayload) -> Result<bool> {
        self.call_api_json("closeForumTopic", payload).await
    }

    pub async fn reopen_forum_topic(&self, payload: &ForumTopicPayload) -> Result<bool> {
        self.call_api_json("reopenForumTopic", payload).await
    }

    pub async fn delete_forum_topic(&self, payload: &ForumTopicPayload) -> Result<bool> {
        self.call_api_json("deleteForumTopic", payload).await
    }

    pub async fn unpin_all_forum_topic_messages(
        &self,
        payload: &ForumTopicPayload,
    ) -> Result<bool> {
        self.call_api_json("unpinAllForumTopicMessages", payload)
            .await
    }

    pub async fn edit_general_forum_topic(
        &self,
        payload: &EditGeneralForumTopicPayload,
    ) -> Result<bool> {
        self.call_api_json("editGeneralForumTopic", payload).await
    }

    pub async fn close_general_forum_topic(&self, payload: &ChatIdPayload) -> Result<bool> {
        self.call_api_json("closeGeneralForumTopic", payload).await
    }

    pub async fn reopen_general_forum_topic(&self, payload: &ChatIdPayload) -> Result<bool> {
        self.call_api_json("reopenGeneralForumTopic", payload).await
    }

    pub async fn hide_general_forum_topic(&self, payload: &ChatIdPayload) -> Result<bool> {
        self.call_api_json("hideGeneralForumTopic", payload).await
    }

    pub async fn unhide_general_forum_topic(&self, payload: &ChatIdPayload) -> Result<bool> {
        self.call_api_json("unhideGeneralForumTopic", payload).await
    }

    pub async fn unpin_all_general_forum_topic_messages(
        &self,
        payload: &ChatIdPayload,
    ) -> Result<bool> {
        self.call_api_json("unpinAllGeneralForumTopicMessages", payload)
            .await
    }

    pub async fn answer_callback_query(
        &self,
        payload: &AnswerCallbackQueryPayload,
    ) -> Result<bool> {
        self.call_api_json("answerCallbackQuery", payload).await
    }

    pub async fn get_user_chat_boosts(
        &self,
        payload: &GetUserChatBoostsPayload,
    ) -> Result<UserChatBoosts> {
        self.call_api_json("getUserChatBoosts", payload).await
    }

    pub async fn get_business_connection(
        &self,
        payload: &GetBusinessConnectionPayload,
    ) -> Result<BusinessConnection> {
        self.call_api_json("getBusinessConnection", payload).await
    }

    pub async fn set_my_commands(&self, payload: &SetMyCommandsPayload) -> Result<bool> {
        self.call_api_json("setMyCommands", payload).await
    }

    pub async fn delete_my_commands(&self, payload: &DeleteMyCommandsPayload) -> Result<bool> {
        self.call_api_json("deleteMyCommands", payload).await
    }

    pub async fn get_my_commands(&self, payload: &GetMyCommandsPayload) -> Result<Vec<BotCommand>> {
        self.call_api_json("getMyCommands", payload).await
    }

    pub async fn set_my_name(&self, payload: &SetMyNamePayload) -> Result<bool> {
        self.call_api_json("setMyName", payload).await
    }

    pub async fn get_my_name(&self, payload: &SetMyNamePayload) -> Result<BotName> {
        self.call_api_json("getMyName", payload).await
    }

    pub async fn set_my_description(&self, payload: &SetMyDescriptionPayload) -> Result<bool> {
        self.call_api_json("setMyDescription", payload).await
    }

    pub async fn get_my_description(
        &self,
        payload: &LanguageCodePayload,
    ) -> Result<BotDescription> {
        self.call_api_json("getMyDescription", payload).await
    }

    pub async fn set_my_short_description(
        &self,
        payload: &SetMyShortDescriptionPayload,
    ) -> Result<bool> {
        self.call_api_json("setMyShortDescription", payload).await
    }

    pub async fn get_my_short_description(
        &self,
        payload: &LanguageCodePayload,
    ) -> Result<BotShortDescription> {
        self.call_api_json("getMyShortDescription", payload).await
    }

    pub async fn set_chat_menu_button(&self, payload: &SetChatMenuButtonPayload) -> Result<bool> {
        self.call_api_json("setChatMenuButton", payload).await
    }

    pub async fn get_chat_menu_button(
        &self,
        payload: &Option<ChatIdPayload>,
    ) -> Result<MenuButton> {
        if let Some(payload) = payload {
            self.call_api_json("getChatMenuButton", payload).await
        } else {
            self.call_api_no_payload("getChatMenuButton").await
        }
    }

    pub async fn set_my_default_administrator_rights(
        &self,
        payload: &SetMyDefaultAdministratorRightsPayload,
    ) -> Result<bool> {
        self.call_api_json("setMyDefaultAdministratorRights", payload)
            .await
    }

    pub async fn get_my_default_administrator_rights(
        &self,
        payload: &GetMyDefaultAdministratorRightsPayload,
    ) -> Result<ChatAdministratorRights> {
        self.call_api_json("getMyDefaultAdministratorRights", payload)
            .await
    }
}
