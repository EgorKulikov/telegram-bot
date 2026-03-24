use crate::requests::*;
use crate::types::*;

/// Use this method to set the bot's profile photo.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SetMyProfilePhoto {
    photo: InputProfilePhoto,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_public: Option<bool>,
}
impl Request for SetMyProfilePhoto {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("setMyProfilePhoto"), self) }
}
impl SetMyProfilePhoto {
    pub fn new(photo: InputProfilePhoto) -> Self {
        SetMyProfilePhoto { photo, is_public: None }
    }
    pub fn is_public(&mut self, is_public: bool) -> &mut Self { self.is_public = Some(is_public); self }
}
