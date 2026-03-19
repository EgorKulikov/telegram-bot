use crate::requests::*;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SendMediaGroup {
    chat_id: ChatRef,
    media: Vec<InputMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<ReplyParameters>,
}

impl Request for SendMediaGroup {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<Vec<MessageOrChannelPost>>;
    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("sendMediaGroup"), self)
    }
}

impl SendMediaGroup {
    pub fn new<C: ToChatRef>(chat: C, media: Vec<InputMedia>) -> Self {
        SendMediaGroup {
            chat_id: chat.to_chat_ref(), media,
            message_thread_id: None, disable_notification: None, protect_content: None,
            reply_parameters: None,
        }
    }
    pub fn disable_notification(&mut self) -> &mut Self { self.disable_notification = Some(true); self }
    pub fn protect_content(&mut self) -> &mut Self { self.protect_content = Some(true); self }
    pub fn reply_parameters(&mut self, params: ReplyParameters) -> &mut Self {
        self.reply_parameters = Some(params); self
    }
}
