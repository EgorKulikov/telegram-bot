use crate::requests::*;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct EditChatInviteLink {
    chat_id: ChatRef,
    invite_link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_date: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_limit: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creates_join_request: Option<bool>,
}

impl Request for EditChatInviteLink {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<ChatInviteLink>;
    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("editChatInviteLink"), self)
    }
}

impl EditChatInviteLink {
    pub fn new<C: ToChatRef, S: Into<String>>(chat: C, invite_link: S) -> Self {
        EditChatInviteLink {
            chat_id: chat.to_chat_ref(), invite_link: invite_link.into(),
            name: None, expire_date: None, member_limit: None, creates_join_request: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct RevokeChatInviteLink {
    chat_id: ChatRef,
    invite_link: String,
}

impl Request for RevokeChatInviteLink {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<ChatInviteLink>;
    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("revokeChatInviteLink"), self)
    }
}

impl RevokeChatInviteLink {
    pub fn new<C: ToChatRef, S: Into<String>>(chat: C, invite_link: S) -> Self {
        RevokeChatInviteLink { chat_id: chat.to_chat_ref(), invite_link: invite_link.into() }
    }
}
