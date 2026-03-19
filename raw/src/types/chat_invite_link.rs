use crate::types::*;

/// Represents an invite link for a chat.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ChatInviteLink {
    /// The invite link. If the link was created by another chat administrator, then the second part of the link will be replaced with “…”.
    pub invite_link: String,
    /// Creator of the link
    pub creator: User,
    /// True, if the link is primary
    pub is_primary: bool,
    /// True, if the link is revoked
    pub is_revoked: bool,
    /// Point in time (Unix timestamp) when the link will expire or has been expired
    pub expire_date: Option<Integer>,
    /// Maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    pub member_limit: Option<Integer>,
    /// Invite link name.
    pub name: Option<String>,
    /// True, if users joining the chat via the link need to be approved by chat administrators.
    pub creates_join_request: Option<bool>,
    /// Number of pending join requests created using this link.
    pub pending_join_request_count: Option<Integer>,
    /// The number of seconds the subscription will be active for before the next payment.
    pub subscription_period: Option<Integer>,
    /// The amount of Telegram Stars a user must pay initially and after each subsequent subscription period to be a member of the chat.
    pub subscription_price: Option<Integer>,
}
