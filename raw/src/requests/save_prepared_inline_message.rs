use crate::requests::*;
use crate::types::*;

/// Stores a message that can be sent by a user of a Mini App.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SavePreparedInlineMessage {
    user_id: UserId,
    result: InlineQueryResult,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_user_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_bot_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_group_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_channel_chats: Option<bool>,
}
impl Request for SavePreparedInlineMessage {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<PreparedInlineMessage>;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("savePreparedInlineMessage"), self) }
}
impl SavePreparedInlineMessage {
    pub fn new<U: ToUserId>(user: U, result: InlineQueryResult) -> Self {
        SavePreparedInlineMessage {
            user_id: user.to_user_id(), result,
            allow_user_chats: None, allow_bot_chats: None, allow_group_chats: None, allow_channel_chats: None,
        }
    }
}

/// Describes an inline message to be sent by a user of a Mini App.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PreparedInlineMessage {
    pub id: String,
    pub expiration_date: Integer,
}
