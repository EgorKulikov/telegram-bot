use crate::requests::*;
use crate::types::*;

/// Use this method to get a sticker set.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetStickerSet {
    name: String,
}

impl Request for GetStickerSet {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<StickerSet>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("getStickerSet"), self)
    }
}

impl GetStickerSet {
    pub fn new<S: Into<String>>(name: S) -> Self {
        GetStickerSet { name: name.into() }
    }
}
