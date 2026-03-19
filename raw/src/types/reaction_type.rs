use crate::types::*;

/// This object describes the type of a reaction.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ReactionType {
    #[serde(rename = "emoji")]
    Emoji { emoji: String },
    #[serde(rename = "custom_emoji")]
    CustomEmoji { custom_emoji_id: String },
    #[serde(rename = "paid")]
    Paid,
}

/// Represents a change of a reaction on a message performed by a user.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MessageReactionUpdated {
    pub chat: Chat,
    pub message_id: Integer,
    pub user: Option<User>,
    pub actor_chat: Option<Chat>,
    pub date: Integer,
    pub old_reaction: Vec<ReactionType>,
    pub new_reaction: Vec<ReactionType>,
}

/// This object represents reaction changes on a message with anonymous reactions.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MessageReactionCountUpdated {
    pub chat: Chat,
    pub message_id: Integer,
    pub date: Integer,
    pub reactions: Vec<ReactionCount>,
}

/// Represents a reaction added to a message along with the number of times it was added.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ReactionCount {
    #[serde(rename = "type")]
    pub type_: ReactionType,
    pub total_count: Integer,
}
