use crate::types::*;

/// This object represents a unique gift that can be saved or transferred.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct UniqueGift {
    pub base_name: Option<String>,
    pub name: Option<String>,
    pub number: Option<Integer>,
    pub model: Option<UniqueGiftModel>,
    pub symbol: Option<UniqueGiftSymbol>,
    pub backdrop: Option<UniqueGiftBackdrop>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct UniqueGiftModel {
    pub name: String,
    pub sticker: Sticker,
    pub rarity_per_mille: Integer,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct UniqueGiftSymbol {
    pub name: String,
    pub sticker: Sticker,
    pub rarity_per_mille: Integer,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct UniqueGiftBackdrop {
    pub name: String,
    pub colors: Option<UniqueGiftBackdropColors>,
    pub rarity_per_mille: Integer,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct UniqueGiftBackdropColors {
    pub center_color: Option<Integer>,
    pub edge_color: Option<Integer>,
    pub symbol_color: Option<Integer>,
    pub text_color: Option<Integer>,
}

/// Information about a gift sent or received by a user or chat.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct GiftInfo {
    pub gift: Gift,
    pub owned_gift_id: Option<String>,
    pub convert_star_count: Option<Integer>,
    pub prepaid_upgrade_star_count: Option<Integer>,
    pub can_be_upgraded: Option<bool>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub is_private: Option<bool>,
}

/// Information about a unique gift received and owned by a user or chat.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct UniqueGiftInfo {
    pub gift: UniqueGift,
    pub origin: Option<String>,
    pub owned_gift_id: Option<String>,
    pub transfer_star_count: Option<Integer>,
}

/// This object describes a gift received and owned by a user or chat.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type")]
pub enum OwnedGift {
    #[serde(rename = "regular")]
    Regular {
        gift: Gift,
        owned_gift_id: Option<String>,
        sender_user: Option<User>,
        send_date: Integer,
        text: Option<String>,
        entities: Option<Vec<MessageEntity>>,
        is_private: Option<bool>,
        is_saved: Option<bool>,
        can_be_upgraded: Option<bool>,
        was_refunded: Option<bool>,
        convert_star_count: Option<Integer>,
        prepaid_upgrade_star_count: Option<Integer>,
    },
    #[serde(rename = "unique")]
    Unique {
        gift: UniqueGift,
        owned_gift_id: Option<String>,
        sender_user: Option<User>,
        send_date: Integer,
        is_saved: Option<bool>,
        can_be_transferred: Option<bool>,
        transfer_star_count: Option<Integer>,
    },
}

/// Contains a list of gifts received and owned by a user or chat.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OwnedGifts {
    pub total_count: Integer,
    pub gifts: Vec<OwnedGift>,
    pub next_offset: Option<String>,
}

/// Describes the types of gifts that can be sent to the user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AcceptedGiftTypes {
    #[serde(default)]
    pub unlimited_gifts: bool,
    #[serde(default)]
    pub limited_gifts: bool,
    #[serde(default)]
    pub unique_gifts: bool,
    #[serde(default)]
    pub premium_subscription: bool,
}
