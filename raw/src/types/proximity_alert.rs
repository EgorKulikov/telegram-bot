use crate::types::*;

/// This object represents the content of a service message, sent whenever a user in the chat
/// triggers a proximity alert set by another user.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ProximityAlertTriggered {
    pub traveler: User,
    pub watcher: User,
    pub distance: Integer,
}
