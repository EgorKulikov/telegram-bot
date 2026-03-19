use crate::requests::*;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct CopyMessages {
    chat_id: ChatRef,
    from_chat_id: ChatRef,
    message_ids: Vec<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remove_caption: Option<bool>,
}

impl Request for CopyMessages {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<Vec<super::copy_message::MessageIdResult>>;
    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("copyMessages"), self)
    }
}

impl CopyMessages {
    pub fn new<C: ToChatRef, F: ToChatRef>(chat: C, from_chat: F, message_ids: Vec<MessageId>) -> Self {
        CopyMessages {
            chat_id: chat.to_chat_ref(), from_chat_id: from_chat.to_chat_ref(), message_ids,
            message_thread_id: None, disable_notification: None, protect_content: None, remove_caption: None,
        }
    }
}
