use crate::requests::*;
use crate::types::*;

/// Use this method to get the list of gifts received and owned by a chat.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetChatGifts {
    chat_id: ChatRef,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<Integer>,
}
impl Request for GetChatGifts {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<OwnedGifts>;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("getChatGifts"), self) }
}
impl GetChatGifts {
    pub fn new<C: ToChatRef>(chat: C) -> Self {
        GetChatGifts { chat_id: chat.to_chat_ref(), offset: None, limit: None }
    }
    pub fn offset<S: Into<String>>(&mut self, offset: S) -> &mut Self { self.offset = Some(offset.into()); self }
    pub fn limit(&mut self, limit: Integer) -> &mut Self { self.limit = Some(limit); self }
}
