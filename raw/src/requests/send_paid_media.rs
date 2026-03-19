use crate::requests::*;
use crate::types::*;

/// Describes the paid media to be sent.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum InputPaidMedia {
    #[serde(rename = "photo")]
    Photo { media: String },
    #[serde(rename = "video")]
    Video {
        media: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<Integer>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<Integer>,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<Integer>,
        #[serde(skip_serializing_if = "Option::is_none")]
        supports_streaming: Option<bool>,
    },
}

/// Use this method to send paid media.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SendPaidMedia {
    chat_id: ChatRef,
    star_count: Integer,
    media: Vec<InputPaidMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<ReplyParameters>,
}
impl Request for SendPaidMedia {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<MessageOrChannelPost>;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("sendPaidMedia"), self) }
}
impl SendPaidMedia {
    pub fn new<C: ToChatRef>(chat: C, star_count: Integer, media: Vec<InputPaidMedia>) -> Self {
        SendPaidMedia {
            chat_id: chat.to_chat_ref(), star_count, media, payload: None, caption: None,
            parse_mode: None, show_caption_above_media: None, disable_notification: None,
            protect_content: None, reply_parameters: None,
        }
    }
}
