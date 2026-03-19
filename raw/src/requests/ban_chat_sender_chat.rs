use crate::requests::*;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct BanChatSenderChat {
    chat_id: ChatRef,
    sender_chat_id: ChatRef,
}

impl Request for BanChatSenderChat {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("banChatSenderChat"), self)
    }
}

impl BanChatSenderChat {
    pub fn new<C: ToChatRef, S: ToChatRef>(chat: C, sender: S) -> Self {
        BanChatSenderChat { chat_id: chat.to_chat_ref(), sender_chat_id: sender.to_chat_ref() }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct UnbanChatSenderChat {
    chat_id: ChatRef,
    sender_chat_id: ChatRef,
}

impl Request for UnbanChatSenderChat {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("unbanChatSenderChat"), self)
    }
}

impl UnbanChatSenderChat {
    pub fn new<C: ToChatRef, S: ToChatRef>(chat: C, sender: S) -> Self {
        UnbanChatSenderChat { chat_id: chat.to_chat_ref(), sender_chat_id: sender.to_chat_ref() }
    }
}
