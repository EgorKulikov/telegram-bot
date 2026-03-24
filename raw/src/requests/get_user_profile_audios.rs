use crate::requests::*;
use crate::types::*;

/// Use this method to get a list of profile audios for a user.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetUserProfileAudios {
    user_id: UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<Integer>,
}
impl Request for GetUserProfileAudios {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<UserProfileAudios>;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("getUserProfileAudios"), self) }
}
impl GetUserProfileAudios {
    pub fn new<U: ToUserId>(user: U) -> Self {
        GetUserProfileAudios { user_id: user.to_user_id(), offset: None, limit: None }
    }
    pub fn offset(&mut self, offset: Integer) -> &mut Self { self.offset = Some(offset); self }
    pub fn limit(&mut self, limit: Integer) -> &mut Self { self.limit = Some(limit); self }
}
