use crate::types::*;

/// This object describes the origin of a message.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type")]
pub enum MessageOrigin {
    #[serde(rename = "user")]
    User {
        date: Integer,
        sender_user: User,
    },
    #[serde(rename = "hidden_user")]
    HiddenUser {
        date: Integer,
        sender_user_name: String,
    },
    #[serde(rename = "chat")]
    Chat {
        date: Integer,
        sender_chat: Chat,
        author_signature: Option<String>,
    },
    #[serde(rename = "channel")]
    Channel {
        date: Integer,
        chat: Chat,
        message_id: Integer,
        author_signature: Option<String>,
    },
}
