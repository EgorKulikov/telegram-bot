use crate::types::*;

/// This object represents a gift that can be sent by the bot.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Gift {
    pub id: String,
    pub sticker: Sticker,
    pub star_count: Integer,
    pub total_count: Option<Integer>,
    pub remaining_count: Option<Integer>,
    pub upgrade_star_count: Option<Integer>,
}

/// This object represent a list of gifts.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Gifts {
    pub gifts: Vec<Gift>,
}
