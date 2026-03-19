use crate::requests::*;
use crate::types::*;

/// Use this method to create an additional invite link for a chat.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct CreateChatInviteLink {
    chat_id: ChatRef,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_date: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    member_limit: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creates_join_request: Option<bool>,
}

impl Request for CreateChatInviteLink {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<ChatInviteLink>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("createChatInviteLink"), self)
    }
}

impl CreateChatInviteLink {
    pub fn new<C: ToChatRef>(chat: C) -> Self {
        CreateChatInviteLink {
            chat_id: chat.to_chat_ref(),
            name: None,
            expire_date: None,
            member_limit: None,
            creates_join_request: None,
        }
    }
}
