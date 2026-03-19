use crate::types::*;
use crate::types::chat_boost_updated::ChatBoostSource;

/// This object represents a boost removed from a chat.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ChatBoostRemoved {
    pub chat: Chat,
    pub boost_id: String,
    pub remove_date: Integer,
    pub source: ChatBoostSource,
}
