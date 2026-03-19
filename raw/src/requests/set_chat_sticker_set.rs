use crate::requests::*;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SetChatStickerSet { chat_id: ChatRef, sticker_set_name: String }

impl Request for SetChatStickerSet {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("setChatStickerSet"), self)
    }
}

impl SetChatStickerSet {
    pub fn new<C: ToChatRef, S: Into<String>>(chat: C, name: S) -> Self {
        SetChatStickerSet { chat_id: chat.to_chat_ref(), sticker_set_name: name.into() }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct DeleteChatStickerSet { chat_id: ChatRef }

impl Request for DeleteChatStickerSet {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("deleteChatStickerSet"), self)
    }
}

impl DeleteChatStickerSet {
    pub fn new<C: ToChatRef>(chat: C) -> Self { DeleteChatStickerSet { chat_id: chat.to_chat_ref() } }
}
