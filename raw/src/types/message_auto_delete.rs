use crate::types::*;

/// This object represents a service message about a change in auto-delete timer settings.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MessageAutoDeleteTimerChanged {
    pub message_auto_delete_time: Integer,
}
