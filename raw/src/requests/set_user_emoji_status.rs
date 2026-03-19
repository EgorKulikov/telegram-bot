use crate::requests::*;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SetUserEmojiStatus {
    user_id: UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji_status_custom_emoji_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji_status_expiration_date: Option<Integer>,
}
impl Request for SetUserEmojiStatus {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("setUserEmojiStatus"), self) }
}
impl SetUserEmojiStatus {
    pub fn new<U: ToUserId>(user: U) -> Self {
        SetUserEmojiStatus { user_id: user.to_user_id(), emoji_status_custom_emoji_id: None, emoji_status_expiration_date: None }
    }
    pub fn emoji<S: Into<String>>(&mut self, id: S) -> &mut Self { self.emoji_status_custom_emoji_id = Some(id.into()); self }
    pub fn expiration_date(&mut self, date: Integer) -> &mut Self { self.emoji_status_expiration_date = Some(date); self }
}
