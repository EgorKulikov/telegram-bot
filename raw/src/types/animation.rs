use crate::types::*;

/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video without sound).
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Animation {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: Integer,
    pub height: Integer,
    pub duration: Integer,
    #[serde(alias = "thumb")]
    pub thumbnail: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<Integer>,
}
