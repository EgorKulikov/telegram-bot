use crate::requests::*;
use crate::types::*;

/// Use this method to delete multiple messages simultaneously.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct DeleteMessages {
    chat_id: ChatRef,
    message_ids: Vec<MessageId>,
}

impl Request for DeleteMessages {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("deleteMessages"), self)
    }
}

impl DeleteMessages {
    pub fn new<C>(chat: C, message_ids: Vec<MessageId>) -> Self
    where
        C: ToChatRef,
    {
        DeleteMessages {
            chat_id: chat.to_chat_ref(),
            message_ids,
        }
    }
}
