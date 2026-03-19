use crate::types::*;

/// This object describes paid media.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type")]
pub enum PaidMedia {
    #[serde(rename = "preview")]
    Preview {
        width: Option<Integer>,
        height: Option<Integer>,
        duration: Option<Integer>,
    },
    #[serde(rename = "photo")]
    Photo {
        photo: Vec<PhotoSize>,
    },
    #[serde(rename = "video")]
    Video {
        video: super::message::Video,
    },
}

/// Describes the paid media added to a message.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PaidMediaInfo {
    pub star_count: Integer,
    pub paid_media: Vec<PaidMedia>,
}

/// This object contains information about a paid media purchase.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PaidMediaPurchased {
    pub from: User,
    pub paid_media_payload: String,
}
