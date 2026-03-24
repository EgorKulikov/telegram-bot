use crate::requests::*;
use crate::types::*;

/// Use this method to approve a suggested post in a channel managed by the bot.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct ApproveSuggestedPost {
    chat_id: ChatRef,
    message_id: MessageId,
}
impl Request for ApproveSuggestedPost {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("approveSuggestedPost"), self) }
}
impl ApproveSuggestedPost {
    pub fn new<C: ToChatRef, M: ToMessageId>(chat: C, message_id: M) -> Self {
        ApproveSuggestedPost { chat_id: chat.to_chat_ref(), message_id: message_id.to_message_id() }
    }
}
