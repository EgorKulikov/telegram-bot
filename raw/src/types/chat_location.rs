use crate::types::*;

/// Represents a location to which a chat is connected.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ChatLocation {
    pub location: Location,
    pub address: String,
}
