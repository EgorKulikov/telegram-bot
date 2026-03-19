use crate::types::*;

/// This object represents a boost added to a chat or changed.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ChatBoostUpdated {
    pub chat: Chat,
    pub boost: ChatBoost,
}

/// This object contains information about a chat boost.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ChatBoost {
    pub boost_id: String,
    pub add_date: Integer,
    pub expiration_date: Integer,
    pub source: ChatBoostSource,
}

/// This object describes the source of a chat boost.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "source")]
pub enum ChatBoostSource {
    #[serde(rename = "premium")]
    Premium { user: User },
    #[serde(rename = "gift_code")]
    GiftCode { user: User },
    #[serde(rename = "giveaway")]
    Giveaway {
        giveaway_message_id: Integer,
        user: Option<User>,
        prize_star_count: Option<Integer>,
        is_unclaimed: Option<bool>,
    },
}
