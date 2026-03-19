use crate::types::*;

/// This object contains information about the quoted part of a message.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TextQuote {
    pub text: String,
    pub entities: Option<Vec<MessageEntity>>,
    pub position: Integer,
    pub is_manual: Option<bool>,
}
