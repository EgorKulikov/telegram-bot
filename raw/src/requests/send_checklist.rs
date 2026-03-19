use crate::requests::*;
use crate::types::*;

/// Use this method to send a checklist message.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SendChecklist {
    chat_id: ChatRef,
    checklist: InputChecklist,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<ReplyParameters>,
}
impl Request for SendChecklist {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<MessageOrChannelPost>;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("sendChecklist"), self) }
}
impl SendChecklist {
    pub fn new<C: ToChatRef>(chat: C, checklist: InputChecklist) -> Self {
        SendChecklist { chat_id: chat.to_chat_ref(), checklist, message_thread_id: None, disable_notification: None, protect_content: None, reply_parameters: None }
    }
}
