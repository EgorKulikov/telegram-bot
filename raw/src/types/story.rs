use crate::types::*;

/// This object represents a story.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Story {
    pub chat: Chat,
    pub id: Integer,
}
