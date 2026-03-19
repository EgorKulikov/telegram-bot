use crate::types::*;

/// This object represents the content of a media message to be sent.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum InputMedia {
    #[serde(rename = "animation")]
    Animation {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<Integer>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<Integer>,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<Integer>,
        #[serde(skip_serializing_if = "Option::is_none")]
        has_spoiler: Option<bool>,
    },
    #[serde(rename = "document")]
    Document {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        #[serde(skip_serializing_if = "Option::is_none")]
        disable_content_type_detection: Option<bool>,
    },
    #[serde(rename = "audio")]
    Audio {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<Integer>,
        #[serde(skip_serializing_if = "Option::is_none")]
        performer: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },
    #[serde(rename = "photo")]
    Photo {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        #[serde(skip_serializing_if = "Option::is_none")]
        has_spoiler: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        show_caption_above_media: Option<bool>,
    },
    #[serde(rename = "video")]
    Video {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<ParseMode>,
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<Integer>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<Integer>,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<Integer>,
        #[serde(skip_serializing_if = "Option::is_none")]
        supports_streaming: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        has_spoiler: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        show_caption_above_media: Option<bool>,
    },
}
