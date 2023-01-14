use crate::{Chat, ChatInviteLink, Integer, User};

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct ChatJoinRequest {
    /// Chat the user belongs to
    pub chat: Chat,
    /// Performer of the action, which resulted in the change
    pub from: User,
    /// Date the change was done in Unix time
    pub date: Integer,
    /// Previous information about the chat member
    pub bio: Option<String>,
    /// Chat invite link, which was used by the user to join the chat; for joining by invite link events only.
    pub invite_link: Option<ChatInviteLink>,
}
