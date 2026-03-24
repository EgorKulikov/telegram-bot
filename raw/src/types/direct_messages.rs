use crate::types::*;

/// Service message: a direct messages topic was created.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DirectMessagesTopic {}

/// Service message: the price for direct messages has changed.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DirectMessagePriceChanged {
    pub direct_message_star_count: Integer,
}
