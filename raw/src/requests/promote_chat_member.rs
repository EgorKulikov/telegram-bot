use crate::requests::*;
use crate::types::*;

/// Use this method to promote or demote a user in a supergroup or a channel.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct PromoteChatMember {
    chat_id: ChatRef,
    user_id: UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_manage_chat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_delete_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_manage_video_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_restrict_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_promote_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_change_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_invite_users: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_post_stories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_edit_stories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_delete_stories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_post_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_edit_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_pin_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_manage_topics: Option<bool>,
}

impl Request for PromoteChatMember {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("promoteChatMember"), self)
    }
}

impl PromoteChatMember {
    pub fn new<C: ToChatRef, U: ToUserId>(chat: C, user: U) -> Self {
        PromoteChatMember {
            chat_id: chat.to_chat_ref(),
            user_id: user.to_user_id(),
            is_anonymous: None, can_manage_chat: None, can_delete_messages: None,
            can_manage_video_chats: None, can_restrict_members: None, can_promote_members: None,
            can_change_info: None, can_invite_users: None, can_post_stories: None,
            can_edit_stories: None, can_delete_stories: None, can_post_messages: None,
            can_edit_messages: None, can_pin_messages: None, can_manage_topics: None,
        }
    }
}
