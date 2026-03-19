use crate::types::*;

/// This object contains information about a message that is being replied to,
/// which may come from another chat or forum topic.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ExternalReplyInfo {
    pub origin: super::message_origin::MessageOrigin,
    pub chat: Option<Chat>,
    pub message_id: Option<Integer>,
    pub link_preview_options: Option<super::link_preview_options::LinkPreviewOptions>,
    pub animation: Option<super::animation::Animation>,
    pub audio: Option<Audio>,
    pub document: Option<Document>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub story: Option<super::story::Story>,
    pub video: Option<Video>,
    pub video_note: Option<VideoNote>,
    pub voice: Option<Voice>,
    pub has_media_spoiler: Option<bool>,
    pub contact: Option<Contact>,
    pub dice: Option<super::dice::Dice>,
    pub giveaway: Option<super::giveaway::Giveaway>,
    pub giveaway_winners: Option<super::giveaway::GiveawayWinners>,
    pub location: Option<Location>,
    pub poll: Option<Poll>,
    pub venue: Option<Venue>,
}
