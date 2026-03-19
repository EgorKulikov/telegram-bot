use crate::requests::*;
use crate::types::*;

/// Transfers a gift received by the bot to another user.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct TransferGift {
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
    owned_gift_id: String,
    new_owner_chat_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    star_count: Option<Integer>,
}
impl Request for TransferGift {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("transferGift"), self) }
}
impl TransferGift {
    pub fn new<S: Into<String>>(owned_gift_id: S, new_owner_chat_id: Integer) -> Self {
        TransferGift { business_connection_id: None, owned_gift_id: owned_gift_id.into(), new_owner_chat_id, star_count: None }
    }
}

/// Upgrades a given regular gift to a unique gift.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct UpgradeGift {
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
    owned_gift_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    keep_original_details: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    star_count: Option<Integer>,
}
impl Request for UpgradeGift {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("upgradeGift"), self) }
}
impl UpgradeGift {
    pub fn new<S: Into<String>>(owned_gift_id: S) -> Self {
        UpgradeGift { business_connection_id: None, owned_gift_id: owned_gift_id.into(), keep_original_details: None, star_count: None }
    }
}

/// Converts a given regular gift to Telegram Stars.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct ConvertGiftToStars {
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
    owned_gift_id: String,
}
impl Request for ConvertGiftToStars {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("convertGiftToStars"), self) }
}
impl ConvertGiftToStars {
    pub fn new<S: Into<String>>(owned_gift_id: S) -> Self {
        ConvertGiftToStars { business_connection_id: None, owned_gift_id: owned_gift_id.into() }
    }
}

/// Sends a gift of a premium subscription to another user.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GiftPremiumSubscription {
    user_id: UserId,
    month_count: Integer,
    star_count: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_parse_mode: Option<ParseMode>,
}
impl Request for GiftPremiumSubscription {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("giftPremiumSubscription"), self) }
}
impl GiftPremiumSubscription {
    pub fn new<U: ToUserId>(user: U, month_count: Integer, star_count: Integer) -> Self {
        GiftPremiumSubscription { user_id: user.to_user_id(), month_count, star_count, text: None, text_parse_mode: None }
    }
}
