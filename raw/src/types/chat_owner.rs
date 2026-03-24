use crate::types::*;

/// Service message: the chat owner has left the chat.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ChatOwnerLeft {
    pub until_date: Option<Integer>,
}

/// Service message: the chat owner has changed.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ChatOwnerChanged {
    pub old_owner: User,
    pub new_owner: User,
}
