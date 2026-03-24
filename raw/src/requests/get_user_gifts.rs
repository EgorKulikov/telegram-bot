use crate::requests::*;
use crate::types::*;

/// Use this method to get the list of gifts received and owned by a user.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetUserGifts {
    user_id: UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<Integer>,
}
impl Request for GetUserGifts {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<OwnedGifts>;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("getUserGifts"), self) }
}
impl GetUserGifts {
    pub fn new<U: ToUserId>(user: U) -> Self {
        GetUserGifts { user_id: user.to_user_id(), offset: None, limit: None }
    }
    pub fn offset<S: Into<String>>(&mut self, offset: S) -> &mut Self { self.offset = Some(offset.into()); self }
    pub fn limit(&mut self, limit: Integer) -> &mut Self { self.limit = Some(limit); self }
}
