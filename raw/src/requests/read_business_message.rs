use crate::requests::*;
use crate::types::*;

/// Use this method to mark an incoming message as read on behalf of a business account.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct ReadBusinessMessage {
    business_connection_id: String,
    chat_id: ChatRef,
    message_id: MessageId,
}
impl Request for ReadBusinessMessage {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("readBusinessMessage"), self) }
}
impl ReadBusinessMessage {
    pub fn new<S: Into<String>, C: ToChatRef, M: ToMessageId>(business_connection_id: S, chat: C, message_id: M) -> Self {
        ReadBusinessMessage {
            business_connection_id: business_connection_id.into(),
            chat_id: chat.to_chat_ref(),
            message_id: message_id.to_message_id(),
        }
    }
}
