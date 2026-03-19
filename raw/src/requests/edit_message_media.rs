use crate::requests::*;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct EditMessageMedia {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    media: InputMedia,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}
impl Request for EditMessageMedia {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<MessageOrChannelPost>;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("editMessageMedia"), self) }
}
impl EditMessageMedia {
    pub fn new<C: ToChatRef, M: ToMessageId>(chat: C, message: M, media: InputMedia) -> Self {
        EditMessageMedia {
            chat_id: Some(chat.to_chat_ref()), message_id: Some(message.to_message_id()),
            inline_message_id: None, media, business_connection_id: None, reply_markup: None,
        }
    }
    pub fn inline<S: Into<String>>(inline_message_id: S, media: InputMedia) -> Self {
        EditMessageMedia {
            chat_id: None, message_id: None, inline_message_id: Some(inline_message_id.into()),
            media, business_connection_id: None, reply_markup: None,
        }
    }
}
