use crate::types::*;

/// This object represents a service message about a video chat scheduled in the chat.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct VideoChatScheduled {
    pub start_date: Integer,
}

/// This object represents a service message about a video chat started in the chat.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct VideoChatStarted {}

/// This object represents a service message about a video chat ended in the chat.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct VideoChatEnded {
    pub duration: Integer,
}

/// This object represents a service message about new members invited to a video chat.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct VideoChatParticipantsInvited {
    pub users: Option<Vec<User>>,
}
