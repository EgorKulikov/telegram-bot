use crate::requests::*;
use crate::types::*;

/// Sends a gift to the given user or channel chat. The gift can't be converted to Telegram Stars by the receiver.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SendGift {
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<UserId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatRef>,
    gift_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pay_for_upgrade: Option<bool>,
}

impl Request for SendGift {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("sendGift"), self)
    }
}

impl SendGift {
    pub fn for_user<U: ToUserId, S: Into<String>>(user: U, gift_id: S) -> Self {
        SendGift {
            user_id: Some(user.to_user_id()),
            chat_id: None,
            gift_id: gift_id.into(),
            text: None,
            text_parse_mode: None,
            pay_for_upgrade: None,
        }
    }

    pub fn for_chat<C: ToChatRef, S: Into<String>>(chat: C, gift_id: S) -> Self {
        SendGift {
            user_id: None,
            chat_id: Some(chat.to_chat_ref()),
            gift_id: gift_id.into(),
            text: None,
            text_parse_mode: None,
            pay_for_upgrade: None,
        }
    }
}

/// Returns the list of gifts that can be sent by the bot to users and channel chats.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetAvailableGifts {}

impl Request for GetAvailableGifts {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<Gifts>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("getAvailableGifts"), self)
    }
}

impl GetAvailableGifts {
    pub fn new() -> Self {
        GetAvailableGifts {}
    }
}
