use crate::requests::*;

/// Use this method to remove the bot's profile photo.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct RemoveMyProfilePhoto {
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_public: Option<bool>,
}
impl Request for RemoveMyProfilePhoto {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("removeMyProfilePhoto"), self) }
}
impl RemoveMyProfilePhoto {
    pub fn new() -> Self {
        RemoveMyProfilePhoto { photo_id: None, is_public: None }
    }
    pub fn photo_id<S: Into<String>>(&mut self, id: S) -> &mut Self { self.photo_id = Some(id.into()); self }
    pub fn is_public(&mut self, is_public: bool) -> &mut Self { self.is_public = Some(is_public); self }
}
