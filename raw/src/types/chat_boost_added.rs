use crate::types::*;

/// This object represents a service message about a user boosting a chat.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ChatBoostAdded {
    pub boost_count: Integer,
}
