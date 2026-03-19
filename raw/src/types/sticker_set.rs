use crate::types::*;

/// This object represents a sticker set.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct StickerSet {
    pub name: String,
    pub title: String,
    pub sticker_type: String,
    pub stickers: Vec<Sticker>,
    #[serde(alias = "thumb")]
    pub thumbnail: Option<PhotoSize>,
}

/// This object describes the position on faces where a mask should be placed by default.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaskPosition {
    pub point: String,
    pub x_shift: Float,
    pub y_shift: Float,
    pub scale: Float,
}

/// This object describes a sticker to be added to a sticker set.
#[derive(Debug, Clone, PartialEq)]
pub struct InputSticker {
    pub sticker: InputFile,
    pub format: String,
    pub emoji_list: Vec<String>,
    pub mask_position: Option<MaskPosition>,
    pub keywords: Option<Vec<String>>,
}
