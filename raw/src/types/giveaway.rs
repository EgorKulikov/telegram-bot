use crate::types::*;

/// This object represents a message about a scheduled giveaway.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Giveaway {
    pub chats: Vec<Chat>,
    pub winners_selection_date: Integer,
    pub winner_count: Integer,
    pub only_new_members: Option<bool>,
    pub has_public_winners: Option<bool>,
    pub prize_description: Option<String>,
    pub country_codes: Option<Vec<String>>,
    pub prize_star_count: Option<Integer>,
    pub premium_subscription_month_count: Option<Integer>,
}

/// This object represents a service message about the creation of a scheduled giveaway.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct GiveawayCreated {
    pub prize_star_count: Option<Integer>,
}

/// This object represents a message about the completion of a giveaway with public winners.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct GiveawayWinners {
    pub chat: Chat,
    pub giveaway_message_id: Integer,
    pub winners_selection_date: Integer,
    pub winner_count: Integer,
    pub winners: Vec<User>,
    pub additional_chat_count: Option<Integer>,
    pub prize_star_count: Option<Integer>,
    pub premium_subscription_month_count: Option<Integer>,
    pub unclaimed_prize_count: Option<Integer>,
    pub only_new_members: Option<bool>,
    pub was_refunded: Option<bool>,
    pub prize_description: Option<String>,
}

/// This object represents a service message about the completion of a giveaway without public winners.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct GiveawayCompleted {
    pub winner_count: Integer,
    pub unclaimed_prize_count: Option<Integer>,
    pub giveaway_message: Option<Box<super::message::MessageOrChannelPost>>,
    pub is_star_giveaway: Option<bool>,
}
