use crate::requests::*;
use crate::types::*;

/// Use this method to clear the list of pinned messages in a chat.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct UnpinAllChatMessages {
    chat_id: ChatRef,
}

impl Request for UnpinAllChatMessages {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("unpinAllChatMessages"), self)
    }
}

impl UnpinAllChatMessages {
    pub fn new<C: ToChatRef>(chat: C) -> Self {
        UnpinAllChatMessages {
            chat_id: chat.to_chat_ref(),
        }
    }
}
