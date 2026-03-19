use crate::requests::*;
use crate::types::*;

/// Use this method to approve a chat join request.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct ApproveChatJoinRequest {
    chat_id: ChatRef,
    user_id: UserId,
}

impl Request for ApproveChatJoinRequest {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("approveChatJoinRequest"), self)
    }
}

impl ApproveChatJoinRequest {
    pub fn new<C: ToChatRef, U: ToUserId>(chat: C, user: U) -> Self {
        ApproveChatJoinRequest {
            chat_id: chat.to_chat_ref(),
            user_id: user.to_user_id(),
        }
    }
}

/// Use this method to decline a chat join request.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct DeclineChatJoinRequest {
    chat_id: ChatRef,
    user_id: UserId,
}

impl Request for DeclineChatJoinRequest {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("declineChatJoinRequest"), self)
    }
}

impl DeclineChatJoinRequest {
    pub fn new<C: ToChatRef, U: ToUserId>(chat: C, user: U) -> Self {
        DeclineChatJoinRequest {
            chat_id: chat.to_chat_ref(),
            user_id: user.to_user_id(),
        }
    }
}
