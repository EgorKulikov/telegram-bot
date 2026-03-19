use crate::requests::*;
use crate::types::*;

/// Use this method to copy messages of any kind.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct CopyMessage {
    chat_id: ChatRef,
    from_chat_id: ChatRef,
    message_id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl Request for CopyMessage {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<MessageIdResult>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("copyMessage"), self)
    }
}

impl CopyMessage {
    pub fn new<C, F, M>(chat: C, from_chat: F, message: M) -> Self
    where
        C: ToChatRef,
        F: ToChatRef,
        M: ToMessageId,
    {
        CopyMessage {
            chat_id: chat.to_chat_ref(),
            from_chat_id: from_chat.to_chat_ref(),
            message_id: message.to_message_id(),
            message_thread_id: None,
            caption: None,
            parse_mode: None,
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
            reply_markup: None,
        }
    }
}

/// Result of copyMessage.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MessageIdResult {
    pub message_id: Integer,
}
