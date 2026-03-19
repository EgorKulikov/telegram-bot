use crate::types::*;

/// This object contains information about the users whose identifiers were shared with the bot.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct UsersShared {
    pub request_id: Integer,
    pub users: Vec<SharedUser>,
}

/// This object contains information about a user that was shared with the bot.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SharedUser {
    pub user_id: Integer,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub photo: Option<Vec<PhotoSize>>,
}

/// This object contains information about a chat that was shared with the bot.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ChatShared {
    pub request_id: Integer,
    pub chat_id: Integer,
    pub title: Option<String>,
    pub username: Option<String>,
    pub photo: Option<Vec<PhotoSize>>,
}
