use crate::requests::*;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetCustomEmojiStickers {
    custom_emoji_ids: Vec<String>,
}
impl Request for GetCustomEmojiStickers {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<Vec<Sticker>>;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("getCustomEmojiStickers"), self) }
}
impl GetCustomEmojiStickers {
    pub fn new(ids: Vec<String>) -> Self { GetCustomEmojiStickers { custom_emoji_ids: ids } }
}
