use crate::types::*;

/// Describes reply parameters for the message that is being sent.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ReplyParameters {
    pub message_id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_entities: Option<Vec<RawMessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_position: Option<Integer>,
}

impl ReplyParameters {
    pub fn new<M: ToMessageId>(message: M) -> Self {
        ReplyParameters {
            message_id: message.to_message_id(),
            chat_id: None,
            allow_sending_without_reply: None,
            quote: None,
            quote_parse_mode: None,
            quote_entities: None,
            quote_position: None,
        }
    }
}
