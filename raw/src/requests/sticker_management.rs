use crate::requests::*;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct DeleteStickerFromSet { sticker: String }
impl Request for DeleteStickerFromSet {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("deleteStickerFromSet"), self) }
}
impl DeleteStickerFromSet {
    pub fn new<S: Into<String>>(sticker: S) -> Self { DeleteStickerFromSet { sticker: sticker.into() } }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SetStickerPositionInSet { sticker: String, position: Integer }
impl Request for SetStickerPositionInSet {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("setStickerPositionInSet"), self) }
}
impl SetStickerPositionInSet {
    pub fn new<S: Into<String>>(sticker: S, position: Integer) -> Self {
        SetStickerPositionInSet { sticker: sticker.into(), position }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SetStickerEmojiList { sticker: String, emoji_list: Vec<String> }
impl Request for SetStickerEmojiList {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("setStickerEmojiList"), self) }
}
impl SetStickerEmojiList {
    pub fn new<S: Into<String>>(sticker: S, emoji_list: Vec<String>) -> Self {
        SetStickerEmojiList { sticker: sticker.into(), emoji_list }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SetStickerKeywords { sticker: String, #[serde(skip_serializing_if = "Option::is_none")] keywords: Option<Vec<String>> }
impl Request for SetStickerKeywords {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("setStickerKeywords"), self) }
}
impl SetStickerKeywords {
    pub fn new<S: Into<String>>(sticker: S) -> Self { SetStickerKeywords { sticker: sticker.into(), keywords: None } }
    pub fn keywords(&mut self, kw: Vec<String>) -> &mut Self { self.keywords = Some(kw); self }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SetStickerMaskPosition { sticker: String, #[serde(skip_serializing_if = "Option::is_none")] mask_position: Option<MaskPosition> }
impl Request for SetStickerMaskPosition {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("setStickerMaskPosition"), self) }
}
impl SetStickerMaskPosition {
    pub fn new<S: Into<String>>(sticker: S) -> Self { SetStickerMaskPosition { sticker: sticker.into(), mask_position: None } }
    pub fn mask_position(&mut self, pos: MaskPosition) -> &mut Self { self.mask_position = Some(pos); self }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SetStickerSetTitle { name: String, title: String }
impl Request for SetStickerSetTitle {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("setStickerSetTitle"), self) }
}
impl SetStickerSetTitle {
    pub fn new<S: Into<String>, T: Into<String>>(name: S, title: T) -> Self {
        SetStickerSetTitle { name: name.into(), title: title.into() }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct DeleteStickerSet { name: String }
impl Request for DeleteStickerSet {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("deleteStickerSet"), self) }
}
impl DeleteStickerSet {
    pub fn new<S: Into<String>>(name: S) -> Self { DeleteStickerSet { name: name.into() } }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct ReplaceStickerInSet {
    user_id: UserId,
    name: String,
    old_sticker: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    sticker: Option<String>,
}
impl Request for ReplaceStickerInSet {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("replaceStickerInSet"), self) }
}
impl ReplaceStickerInSet {
    pub fn new<U: ToUserId, S: Into<String>, O: Into<String>>(user: U, name: S, old_sticker: O) -> Self {
        ReplaceStickerInSet { user_id: user.to_user_id(), name: name.into(), old_sticker: old_sticker.into(), sticker: None }
    }
}
