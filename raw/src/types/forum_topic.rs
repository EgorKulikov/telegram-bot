use crate::types::*;

/// This object represents a forum topic.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ForumTopic {
    pub message_thread_id: Integer,
    pub name: String,
    pub icon_color: Integer,
    pub icon_custom_emoji_id: Option<String>,
}

/// This object represents a service message about a new forum topic created in the chat.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ForumTopicCreated {
    pub name: String,
    pub icon_color: Integer,
    pub icon_custom_emoji_id: Option<String>,
}

/// This object represents a service message about an edited forum topic.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ForumTopicEdited {
    pub name: Option<String>,
    pub icon_custom_emoji_id: Option<String>,
}

/// This object represents a service message about a forum topic closed in the chat.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ForumTopicClosed {}

/// This object represents a service message about a forum topic reopened in the chat.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ForumTopicReopened {}

/// This object represents a service message about General forum topic hidden in the chat.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct GeneralForumTopicHidden {}

/// This object represents a service message about General forum topic unhidden in the chat.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct GeneralForumTopicUnhidden {}
