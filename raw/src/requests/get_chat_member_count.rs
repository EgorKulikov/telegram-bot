use crate::requests::*;
use crate::types::*;

/// Use this method to get the number of members in a chat. Replaces getChatMembersCount.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetChatMemberCount { chat_id: ChatRef }

impl Request for GetChatMemberCount {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<Integer>;
    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("getChatMemberCount"), self)
    }
}

impl GetChatMemberCount {
    pub fn new<C: ToChatRef>(chat: C) -> Self { GetChatMemberCount { chat_id: chat.to_chat_ref() } }
}
