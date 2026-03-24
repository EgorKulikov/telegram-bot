use serde::de::{Deserialize, Deserializer, Error};
use serde::ser::Serialize;

use crate::types::*;
use crate::url::*;

/// This object represents a chat message or a channel post.
#[derive(Debug, Clone, PartialEq)]
pub enum MessageOrChannelPost {
    Message(Message),
    ChannelPost(ChannelPost),
}

/// This object represents a chat message.
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    /// Unique message identifier inside this chat.
    pub id: MessageId,
    /// Sender, can be empty for messages sent to channels.
    pub from: Option<User>,
    /// Date the message was sent in Unix time.
    pub date: Integer,
    /// Conversation the message belongs to.
    pub chat: MessageChat,
    /// Information about the original message.
    pub forward: Option<Forward>,
    /// For replies, the original message. Note that the Message object in this field will not
    /// contain further reply_to_message fields even if it itself is a reply.
    pub reply_to_message: Option<Box<MessageOrChannelPost>>,
    /// Date the message was last edited in Unix time.
    pub edit_date: Option<Integer>,
    /// Kind of the message.
    pub kind: MessageKind,
}

/// This object represents a channel message.
#[derive(Debug, Clone, PartialEq)]
pub struct ChannelPost {
    /// Unique message identifier inside this chat.
    pub id: MessageId,
    /// Date the message was sent in Unix time.
    pub date: Integer,
    /// Conversation the message belongs to.
    pub chat: Channel,
    /// Information about the original message.
    pub forward: Option<Forward>,
    /// For replies, the original message. Note that the Message object in this field will not
    /// contain further reply_to_message fields even if it itself is a reply.
    pub reply_to_message: Option<Box<MessageOrChannelPost>>,
    /// Date the message was last edited in Unix time.
    pub edit_date: Option<Integer>,
    /// Kind of the message.
    pub kind: MessageKind,
}

/// Information about the original message.
#[derive(Debug, Clone, PartialEq)]
pub struct Forward {
    /// Date the original message was sent in Unix time
    pub date: Integer,
    /// Sender of the original message.
    pub from: ForwardFrom,
}

/// Information about the source of the original message.
#[derive(Debug, Clone, PartialEq)]
pub enum ForwardFrom {
    /// Sender of the original message.
    User {
        /// Sender of the original message.
        user: User,
    },
    /// For messages forwarded from a channel, information about the original channel.
    Channel {
        /// Original channel.
        channel: Channel,
        /// Identifier of the original message in the channel
        message_id: Integer,
    },
    ChannelHiddenUser {
        sender_name: String,
    },
    /// For messages forwarded from a hidden admin
    HiddenGroupAdmin {
        /// Original group
        chat_id: SupergroupId,
        /// Group title
        title: String,
    },
}

/// Kind of the message.
#[derive(Debug, Clone, PartialEq)]
pub enum MessageKind {
    /// Text message.
    Text {
        /// Actual UTF-8 text of the message, 0-4096 characters.
        data: String,
        /// Special entities like usernames, URLs, bot commands, etc. that appear in the text
        entities: Vec<MessageEntity>,
    },
    /// Message is an audio file.
    Audio {
        /// Information about the file.
        data: Audio,
    },
    /// Message is a general file.
    Document {
        /// Information about the file.
        data: Document,
        /// Caption for the document, 0-200 characters.
        caption: Option<String>,
    },
    /// Message is a photo.
    Photo {
        /// Available sizes of the photo.
        data: Vec<PhotoSize>,
        /// Caption for the photo, 0-200 characters.
        caption: Option<String>,
        /// The unique identifier of a media message group this message belongs to.
        media_group_id: Option<String>,
    },
    /// Message is a sticker.
    Sticker {
        /// Information about the sticker.
        data: Sticker,
    },
    /// Message is a video.
    Video {
        /// Information about the video.
        data: Video,
        /// Caption for the video, 0-200 characters.
        caption: Option<String>,
        /// The unique identifier of a media message group this message belongs to.
        media_group_id: Option<String>,
    },
    /// Message is a voice message.
    Voice {
        /// Information about the file.
        data: Voice,
    },
    /// Message is a video note.
    VideoNote {
        /// Information about the file.
        data: VideoNote,
    },
    /// Message is a shared contact.
    Contact {
        /// Information about the contact.
        data: Contact,
    },
    /// Message is a shared location.
    Location {
        /// Information about the location.
        data: Location,
    },
    /// Message is a poll.
    Poll {
        /// Information about the poll.
        data: Poll,
    },
    /// Message is a venue.
    Venue {
        /// Information about the venue.
        data: Venue,
    },
    /// New members that were added to the group or supergroup and
    /// information about them (the bot itself may be one of these members)
    NewChatMembers {
        /// Information about user (this member may be the bot itself).
        data: Vec<User>,
    },
    /// A member was removed from the group.
    LeftChatMember {
        /// Information about user (this member may be the bot itself).
        data: User,
    },
    /// New chat title.
    NewChatTitle {
        /// A chat title was changed to this value.
        data: String,
    },
    /// New chat photo.
    NewChatPhoto {
        /// A chat photo was change to this value.
        data: Vec<PhotoSize>,
    },
    /// Service message: the chat photo was deleted.
    DeleteChatPhoto,
    /// Service message: the group has been created.
    GroupChatCreated,
    /// Service message: the supergroup has been created. This field can‘t be received in a
    /// message coming through updates, because bot can’t be a member of a supergroup when
    /// it is created. It can only be found in reply_to_message if someone replies to a very
    /// first message in a directly created supergroup.
    SupergroupChatCreated,
    /// Service message: the channel has been created. This field can‘t be received in a message
    /// coming through updates, because bot can’t be a member of a channel when it is created.
    /// It can only be found in reply_to_message if someone replies
    /// to a very first message in a channel.
    ChannelChatCreated,
    /// The group has been migrated to a supergroup.
    MigrateToChatId {
        /// Supergroup chat identifier.
        data: Integer,
    },
    /// The supergroup has been migrated from a group.
    MigrateFromChatId {
        /// Group chat identifier.
        data: Integer,
    },
    /// Specified message was pinned.
    PinnedMessage {
        data: Box<MessageOrChannelPost>,
    },
    /// Message is an animation (GIF).
    Animation {
        data: super::animation::Animation,
        caption: Option<String>,
        media_group_id: Option<String>,
    },
    /// Message is a dice with random value.
    Dice {
        data: super::dice::Dice,
    },
    /// Message is a game.
    Game {
        raw: RawMessage,
    },
    /// Service message: video chat scheduled.
    VideoChatScheduled {
        data: super::video_chat::VideoChatScheduled,
    },
    /// Service message: video chat started.
    VideoChatStarted,
    /// Service message: video chat ended.
    VideoChatEnded {
        data: super::video_chat::VideoChatEnded,
    },
    /// Service message: new participants invited to a video chat.
    VideoChatParticipantsInvited {
        data: super::video_chat::VideoChatParticipantsInvited,
    },
    /// Service message: auto-delete timer settings changed.
    MessageAutoDeleteTimerChanged {
        data: super::message_auto_delete::MessageAutoDeleteTimerChanged,
    },
    /// Service message: a user was allowed to write messages.
    WriteAccessAllowed {
        data: super::write_access_allowed::WriteAccessAllowed,
    },
    /// Service message: forum topic created.
    ForumTopicCreated {
        data: super::forum_topic::ForumTopicCreated,
    },
    /// Service message: forum topic edited.
    ForumTopicEdited {
        data: super::forum_topic::ForumTopicEdited,
    },
    /// Service message: forum topic closed.
    ForumTopicClosed,
    /// Service message: forum topic reopened.
    ForumTopicReopened,
    /// Service message: General forum topic hidden.
    GeneralForumTopicHidden,
    /// Service message: General forum topic unhidden.
    GeneralForumTopicUnhidden,
    /// Service message: a proximity alert was triggered.
    ProximityAlertTriggered {
        data: super::proximity_alert::ProximityAlertTriggered,
    },
    /// Service message: users were shared with the bot.
    UsersShared {
        data: super::users_shared::UsersShared,
    },
    /// Service message: a chat was shared with the bot.
    ChatShared {
        data: super::users_shared::ChatShared,
    },
    /// Message is a scheduled giveaway.
    Giveaway {
        data: super::giveaway::Giveaway,
    },
    /// Service message: a giveaway was created.
    GiveawayCreated {
        data: super::giveaway::GiveawayCreated,
    },
    /// Message about the completion of a giveaway with public winners.
    GiveawayWinners {
        data: super::giveaway::GiveawayWinners,
    },
    /// Service message: a giveaway without public winners was completed.
    GiveawayCompleted {
        data: super::giveaway::GiveawayCompleted,
    },
    /// Message is a forwarded story.
    Story {
        data: super::story::Story,
    },
    /// Message contains paid media.
    PaidMedia {
        data: super::paid_media::PaidMediaInfo,
    },
    /// Service message: chat background was set.
    ChatBackgroundSet {
        data: super::chat_background::ChatBackground,
    },
    /// The channel post was sent from a web app.
    WebAppData {
        data: super::web_app::WebAppData,
    },
    /// Service message: user boosted the chat.
    BoostAdded {
        boost_count: Integer,
    },
    /// Message is an invoice for a payment.
    Invoice {
        data: super::successful_payment::Invoice,
    },
    /// Service message: successful payment.
    SuccessfulPayment {
        data: super::successful_payment::SuccessfulPayment,
    },
    /// Service message: refunded payment.
    RefundedPayment {
        data: super::successful_payment::RefundedPayment,
    },
    /// Service message: a gift was sent or received.
    Gift {
        data: super::unique_gift::GiftInfo,
    },
    /// Service message: a unique gift was sent or received.
    UniqueGift {
        data: super::unique_gift::UniqueGiftInfo,
    },
    /// Service message: a gift upgrade was sent.
    GiftUpgradeSent {
        data: super::unique_gift::GiftInfo,
    },
    /// Service message: the price for paid messages has changed.
    PaidMessagePriceChanged {
        data: super::successful_payment::PaidMessagePriceChanged,
    },
    /// Message is a checklist.
    Checklist {
        data: super::checklist::Checklist,
    },
    /// Service message: checklist tasks were completed.
    ChecklistTasksDone {
        data: super::checklist::ChecklistTasksDone,
    },
    /// Service message: checklist tasks were added.
    ChecklistTasksAdded {
        data: super::checklist::ChecklistTasksAdded,
    },
    /// Service message: the domain name of the website on which the user has logged in.
    ConnectedWebsite {
        data: String,
    },
    /// Service message: the chat owner has left.
    ChatOwnerLeft {
        data: super::chat_owner::ChatOwnerLeft,
    },
    /// Service message: the chat owner has changed.
    ChatOwnerChanged {
        data: super::chat_owner::ChatOwnerChanged,
    },
    /// Service message: the price for direct messages has changed.
    DirectMessagePriceChanged {
        data: super::direct_messages::DirectMessagePriceChanged,
    },
    /// Service message: a suggested post was approved.
    SuggestedPostApproved {
        data: super::suggested_post::SuggestedPostApproved,
    },
    /// Service message: a suggested post approval failed.
    SuggestedPostApprovalFailed {
        data: super::suggested_post::SuggestedPostApprovalFailed,
    },
    /// Service message: a suggested post was declined.
    SuggestedPostDeclined {
        data: super::suggested_post::SuggestedPostDeclined,
    },
    /// Service message: a suggested post was paid.
    SuggestedPostPaid {
        data: super::suggested_post::SuggestedPostPaid,
    },
    /// Service message: a suggested post was refunded.
    SuggestedPostRefunded {
        data: super::suggested_post::SuggestedPostRefunded,
    },
    #[doc(hidden)]
    Unknown { raw: RawMessage },
}

impl Message {
    fn from_raw_message(raw: RawMessage) -> Result<Self, String> {
        let id = raw.message_id;
        let from = raw.from.clone();
        let date = raw.date;
        let chat = match raw.chat.clone() {
            Chat::Private(x) => MessageChat::Private(x),
            Chat::Group(x) => MessageChat::Group(x),
            Chat::Supergroup(x) => MessageChat::Supergroup(x),
            Chat::Unknown(x) => MessageChat::Unknown(x),
            Chat::Channel(_) => return Err(format!("Channel chat in Message")),
        };

        let reply_to_message = raw.reply_to_message.clone();
        let edit_date = raw.edit_date;

        let forward = match (
            raw.forward_date,
            &raw.forward_from,
            &raw.forward_from_chat,
            raw.forward_from_message_id,
            &raw.forward_sender_name,
        ) {
            (None, &None, &None, None, &None) => None,
            (Some(date), &Some(ref from), &None, None, &None) => Some(Forward {
                date: date,
                from: ForwardFrom::User { user: from.clone() },
            }),
            (Some(date), &None, &Some(Chat::Channel(ref channel)), Some(message_id), &None) => {
                Some(Forward {
                    date: date,
                    from: ForwardFrom::Channel {
                        channel: channel.clone(),
                        message_id: message_id,
                    },
                })
            }
            (Some(date), &None, &None, None, &Some(ref sender_name)) => Some(Forward {
                date,
                from: ForwardFrom::ChannelHiddenUser {
                    sender_name: sender_name.clone(),
                },
            }),
            (
                Some(date),
                None,
                Some(Chat::Supergroup(Supergroup {
                    id: chat_id, title, ..
                })),
                None,
                None,
            ) => Some(Forward {
                date,
                from: ForwardFrom::HiddenGroupAdmin {
                    chat_id: chat_id.clone(),
                    title: title.clone(),
                },
            }),
            _ => None,
        };

        let make_message = |kind| {
            Ok(Message {
                id: id.into(),
                from: from,
                date: date,
                chat: chat,
                forward: forward,
                reply_to_message: reply_to_message,
                edit_date: edit_date,
                kind: kind,
            })
        };

        macro_rules! maybe_field {
            ($name:ident, $variant:ident) => {{
                if let Some(val) = raw.$name {
                    return make_message(MessageKind::$variant { data: val });
                }
            }};
        }

        macro_rules! maybe_field_with_caption {
            ($name:ident, $variant:ident) => {{
                if let Some(val) = raw.$name {
                    return make_message(MessageKind::$variant {
                        data: val,
                        caption: raw.caption,
                    });
                }
            }};
        }

        macro_rules! maybe_field_with_caption_and_group {
            ($name:ident, $variant:ident) => {{
                if let Some(val) = raw.$name {
                    return make_message(MessageKind::$variant {
                        data: val,
                        caption: raw.caption,
                        media_group_id: raw.media_group_id,
                    });
                }
            }};
        }

        macro_rules! maybe_true_field {
            ($name:ident, $variant:ident) => {{
                if let Some(True) = raw.$name {
                    return make_message(MessageKind::$variant);
                }
            }};
        }

        if let Some(text) = raw.text {
            let entities = raw.entities.unwrap_or_else(Vec::new);
            return make_message(MessageKind::Text {
                data: text,
                entities: entities,
            });
        }

        maybe_field!(audio, Audio);
        maybe_field_with_caption!(document, Document);
        maybe_field_with_caption_and_group!(photo, Photo);
        maybe_field!(sticker, Sticker);
        maybe_field_with_caption_and_group!(video, Video);
        maybe_field!(voice, Voice);
        maybe_field!(video_note, VideoNote);
        maybe_field!(contact, Contact);
        maybe_field!(location, Location);
        maybe_field!(poll, Poll);
        maybe_field!(venue, Venue);
        maybe_field!(new_chat_members, NewChatMembers);
        maybe_field!(left_chat_member, LeftChatMember);
        maybe_field!(new_chat_title, NewChatTitle);
        maybe_field!(new_chat_photo, NewChatPhoto);
        maybe_true_field!(delete_chat_photo, DeleteChatPhoto);
        maybe_true_field!(delete_chat_photo, DeleteChatPhoto);
        maybe_true_field!(group_chat_created, GroupChatCreated);
        maybe_true_field!(supergroup_chat_created, SupergroupChatCreated);
        maybe_true_field!(channel_chat_created, ChannelChatCreated);
        maybe_field!(migrate_to_chat_id, MigrateToChatId);
        maybe_field!(migrate_from_chat_id, MigrateFromChatId);
        maybe_field!(pinned_message, PinnedMessage);
        maybe_field_with_caption_and_group!(animation, Animation);
        maybe_field!(dice, Dice);
        maybe_field!(video_chat_scheduled, VideoChatScheduled);
        if raw.video_chat_started.is_some() {
            return make_message(MessageKind::VideoChatStarted);
        }
        maybe_field!(video_chat_ended, VideoChatEnded);
        maybe_field!(video_chat_participants_invited, VideoChatParticipantsInvited);
        maybe_field!(message_auto_delete_timer_changed, MessageAutoDeleteTimerChanged);
        maybe_field!(write_access_allowed, WriteAccessAllowed);
        maybe_field!(forum_topic_created, ForumTopicCreated);
        maybe_field!(forum_topic_edited, ForumTopicEdited);
        if raw.forum_topic_closed.is_some() {
            return make_message(MessageKind::ForumTopicClosed);
        }
        if raw.forum_topic_reopened.is_some() {
            return make_message(MessageKind::ForumTopicReopened);
        }
        if raw.general_forum_topic_hidden.is_some() {
            return make_message(MessageKind::GeneralForumTopicHidden);
        }
        if raw.general_forum_topic_unhidden.is_some() {
            return make_message(MessageKind::GeneralForumTopicUnhidden);
        }
        maybe_field!(proximity_alert_triggered, ProximityAlertTriggered);
        maybe_field!(users_shared, UsersShared);
        maybe_field!(chat_shared, ChatShared);
        maybe_field!(giveaway, Giveaway);
        maybe_field!(giveaway_created, GiveawayCreated);
        maybe_field!(giveaway_winners, GiveawayWinners);
        maybe_field!(giveaway_completed, GiveawayCompleted);
        maybe_field!(story, Story);
        maybe_field!(paid_media, PaidMedia);
        maybe_field!(chat_background_set, ChatBackgroundSet);
        maybe_field!(web_app_data, WebAppData);
        if let Some(count) = raw.boost_added {
            return make_message(MessageKind::BoostAdded { boost_count: count });
        }
        maybe_field!(invoice, Invoice);
        maybe_field!(successful_payment, SuccessfulPayment);
        maybe_field!(refunded_payment, RefundedPayment);
        maybe_field!(gift, Gift);
        maybe_field!(unique_gift, UniqueGift);
        maybe_field!(gift_upgrade_sent, GiftUpgradeSent);
        maybe_field!(paid_message_price_changed, PaidMessagePriceChanged);
        maybe_field!(checklist, Checklist);
        maybe_field!(checklist_tasks_done, ChecklistTasksDone);
        maybe_field!(checklist_tasks_added, ChecklistTasksAdded);
        maybe_field!(connected_website, ConnectedWebsite);
        maybe_field!(chat_owner_left, ChatOwnerLeft);
        maybe_field!(chat_owner_changed, ChatOwnerChanged);
        maybe_field!(direct_message_price_changed, DirectMessagePriceChanged);
        maybe_field!(suggested_post_approved, SuggestedPostApproved);
        maybe_field!(suggested_post_approval_failed, SuggestedPostApprovalFailed);
        maybe_field!(suggested_post_declined, SuggestedPostDeclined);
        maybe_field!(suggested_post_paid, SuggestedPostPaid);
        maybe_field!(suggested_post_refunded, SuggestedPostRefunded);

        make_message(MessageKind::Unknown { raw: raw })
    }
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Message, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: RawMessage = Deserialize::deserialize(deserializer)?;

        Self::from_raw_message(raw).map_err(|err| D::Error::custom(err))
    }
}

impl ChannelPost {
    fn from_raw_message(raw: RawMessage) -> Result<Self, String> {
        let id = raw.message_id;
        let date = raw.date;
        let chat = match raw.chat.clone() {
            Chat::Channel(channel) => channel,
            _ => return Err(format!("Expected channel chat type for ChannelMessage")),
        };
        let reply_to_message = raw.reply_to_message.clone();
        let edit_date = raw.edit_date;

        let forward = match (
            raw.forward_date,
            &raw.forward_from,
            &raw.forward_from_chat,
            raw.forward_from_message_id,
            &raw.forward_sender_name,
        ) {
            (None, &None, &None, None, &None) => None,
            (Some(date), &Some(ref from), &None, None, &None) => Some(Forward {
                date: date,
                from: ForwardFrom::User { user: from.clone() },
            }),
            (Some(date), &None, &Some(Chat::Channel(ref channel)), Some(message_id), &None) => {
                Some(Forward {
                    date: date,
                    from: ForwardFrom::Channel {
                        channel: channel.clone(),
                        message_id: message_id,
                    },
                })
            }
            (Some(date), &None, &None, None, &Some(ref sender_name)) => Some(Forward {
                date,
                from: ForwardFrom::ChannelHiddenUser {
                    sender_name: sender_name.clone(),
                },
            }),
            (
                Some(date),
                None,
                Some(Chat::Supergroup(Supergroup {
                    id: chat_id, title, ..
                })),
                None,
                None,
            ) => Some(Forward {
                date,
                from: ForwardFrom::HiddenGroupAdmin {
                    chat_id: chat_id.clone(),
                    title: title.clone(),
                },
            }),
            _ => None,
        };

        let make_message = |kind| {
            Ok(ChannelPost {
                id: id.into(),
                date: date,
                chat: chat,
                forward: forward,
                reply_to_message: reply_to_message,
                edit_date: edit_date,
                kind: kind,
            })
        };

        macro_rules! maybe_field {
            ($name:ident, $variant:ident) => {{
                if let Some(val) = raw.$name {
                    return make_message(MessageKind::$variant { data: val });
                }
            }};
        }

        macro_rules! maybe_field_with_caption {
            ($name:ident, $variant:ident) => {{
                if let Some(val) = raw.$name {
                    return make_message(MessageKind::$variant {
                        data: val,
                        caption: raw.caption,
                    });
                }
            }};
        }

        macro_rules! maybe_field_with_caption_and_group {
            ($name:ident, $variant:ident) => {{
                if let Some(val) = raw.$name {
                    return make_message(MessageKind::$variant {
                        data: val,
                        caption: raw.caption,
                        media_group_id: raw.media_group_id,
                    });
                }
            }};
        }

        macro_rules! maybe_true_field {
            ($name:ident, $variant:ident) => {{
                if let Some(True) = raw.$name {
                    return make_message(MessageKind::$variant);
                }
            }};
        }

        if let Some(text) = raw.text {
            let entities = raw.entities.unwrap_or_else(Vec::new);
            return make_message(MessageKind::Text {
                data: text,
                entities: entities,
            });
        }

        maybe_field!(audio, Audio);
        maybe_field_with_caption!(document, Document);
        maybe_field_with_caption_and_group!(photo, Photo);
        maybe_field!(sticker, Sticker);
        maybe_field_with_caption_and_group!(video, Video);
        maybe_field!(voice, Voice);
        maybe_field!(video_note, VideoNote);
        maybe_field!(contact, Contact);
        maybe_field!(location, Location);
        maybe_field!(poll, Poll);
        maybe_field!(venue, Venue);
        maybe_field!(new_chat_members, NewChatMembers);
        maybe_field!(left_chat_member, LeftChatMember);
        maybe_field!(new_chat_title, NewChatTitle);
        maybe_field!(new_chat_photo, NewChatPhoto);
        maybe_true_field!(delete_chat_photo, DeleteChatPhoto);
        maybe_true_field!(delete_chat_photo, DeleteChatPhoto);
        maybe_true_field!(group_chat_created, GroupChatCreated);
        maybe_true_field!(supergroup_chat_created, SupergroupChatCreated);
        maybe_true_field!(channel_chat_created, ChannelChatCreated);
        maybe_field!(migrate_to_chat_id, MigrateToChatId);
        maybe_field!(migrate_from_chat_id, MigrateFromChatId);
        maybe_field!(pinned_message, PinnedMessage);
        maybe_field_with_caption_and_group!(animation, Animation);
        maybe_field!(dice, Dice);
        if raw.video_chat_started.is_some() {
            return make_message(MessageKind::VideoChatStarted);
        }
        maybe_field!(video_chat_ended, VideoChatEnded);
        maybe_field!(giveaway, Giveaway);
        maybe_field!(giveaway_created, GiveawayCreated);
        maybe_field!(giveaway_winners, GiveawayWinners);
        maybe_field!(giveaway_completed, GiveawayCompleted);
        maybe_field!(story, Story);
        maybe_field!(paid_media, PaidMedia);
        maybe_field!(invoice, Invoice);
        maybe_field!(successful_payment, SuccessfulPayment);
        maybe_field!(refunded_payment, RefundedPayment);
        maybe_field!(gift, Gift);
        maybe_field!(unique_gift, UniqueGift);
        maybe_field!(gift_upgrade_sent, GiftUpgradeSent);
        maybe_field!(paid_message_price_changed, PaidMessagePriceChanged);
        maybe_field!(checklist, Checklist);
        maybe_field!(checklist_tasks_done, ChecklistTasksDone);
        maybe_field!(checklist_tasks_added, ChecklistTasksAdded);
        maybe_field!(connected_website, ConnectedWebsite);
        maybe_field!(chat_owner_left, ChatOwnerLeft);
        maybe_field!(chat_owner_changed, ChatOwnerChanged);
        maybe_field!(direct_message_price_changed, DirectMessagePriceChanged);
        maybe_field!(suggested_post_approved, SuggestedPostApproved);
        maybe_field!(suggested_post_approval_failed, SuggestedPostApprovalFailed);
        maybe_field!(suggested_post_declined, SuggestedPostDeclined);
        maybe_field!(suggested_post_paid, SuggestedPostPaid);
        maybe_field!(suggested_post_refunded, SuggestedPostRefunded);

        make_message(MessageKind::Unknown { raw: raw })
    }
}

impl<'de> Deserialize<'de> for ChannelPost {
    // TODO(knsd): Remove .clone()
    fn deserialize<D>(deserializer: D) -> Result<ChannelPost, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: RawMessage = Deserialize::deserialize(deserializer)?;

        Self::from_raw_message(raw).map_err(|err| D::Error::custom(err))
    }
}

impl<'de> Deserialize<'de> for MessageOrChannelPost {
    // TODO(knsd): Remove .clone()
    fn deserialize<D>(deserializer: D) -> Result<MessageOrChannelPost, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: RawMessage = Deserialize::deserialize(deserializer)?;
        let is_channel = match raw.chat {
            Chat::Channel(_) => true,
            _ => false,
        };

        let res = if is_channel {
            ChannelPost::from_raw_message(raw).map(MessageOrChannelPost::ChannelPost)
        } else {
            Message::from_raw_message(raw).map(MessageOrChannelPost::Message)
        };

        res.map_err(|err| D::Error::custom(err))
    }
}

/// This object represents a message. Directly mapped.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct RawMessage {
    /// Unique message identifier inside this chat.
    pub message_id: Integer,
    /// Unique identifier of a message thread to which the message belongs.
    pub message_thread_id: Option<Integer>,
    /// Sender, can be empty for messages sent to channels.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// If the sender of the message boosted the chat, the number of boosts added by the user.
    pub sender_boost_count: Option<Integer>,
    /// The bot that actually sent the message on behalf of the business account.
    pub sender_business_bot: Option<User>,
    /// Date the message was sent in Unix time.
    pub date: Integer,
    /// Unique identifier of the business connection from which the message was received.
    pub business_connection_id: Option<String>,
    /// Conversation the message belongs to.
    pub chat: Chat,
    /// Information about the original message for forwarded messages.
    pub forward_origin: Option<super::message_origin::MessageOrigin>,
    /// For forwarded messages, sender of the original message.
    pub forward_from: Option<User>,
    /// For messages forwarded from a channel, information about the original channel.
    pub forward_from_chat: Option<Chat>,
    /// For forwarded channel posts, identifier of the original message in the channel.
    pub forward_from_message_id: Option<Integer>,
    /// For forwarded messages, date the original message was sent in Unix time.
    pub forward_date: Option<Integer>,
    /// Forward from channel by a hidden user.
    pub forward_sender_name: Option<String>,
    /// True, if the message is sent to a forum topic.
    pub is_topic_message: Option<bool>,
    /// True, if the message is a channel post that was automatically forwarded.
    pub is_automatic_forward: Option<bool>,
    /// For replies, the original message.
    pub reply_to_message: Option<Box<MessageOrChannelPost>>,
    /// Information about the message that is being replied to, which may come from another chat.
    pub external_reply: Option<super::external_reply_info::ExternalReplyInfo>,
    /// For replies that quote part of the original message, the quoted part of the message.
    pub quote: Option<super::text_quote::TextQuote>,
    /// For replies to a story, the original story.
    pub reply_to_story: Option<super::story::Story>,
    /// Bot through which the message was sent.
    pub via_bot: Option<User>,
    /// Date the message was last edited in Unix time.
    pub edit_date: Option<Integer>,
    /// True, if the message can’t be forwarded.
    pub has_protected_content: Option<bool>,
    /// True, if the message was sent by an implicit action.
    pub is_from_offline: Option<bool>,
    /// The unique identifier of a media message group this message belongs to.
    pub media_group_id: Option<String>,
    /// Signature of the post author for messages in channels.
    pub author_signature: Option<String>,
    /// For text messages, the actual UTF-8 text of the message, 0-4096 characters.
    pub text: Option<String>,
    /// For text messages, special entities like usernames, URLs, bot commands, etc.
    pub entities: Option<Vec<MessageEntity>>,
    /// Options used for link preview generation for the message.
    pub link_preview_options: Option<super::link_preview_options::LinkPreviewOptions>,
    /// Unique identifier of the message effect added to the message.
    pub effect_id: Option<String>,
    /// Message is an animation, information about the animation.
    pub animation: Option<super::animation::Animation>,
    /// Message is an audio file, information about the file.
    pub audio: Option<Audio>,
    /// Message is a general file, information about the file.
    pub document: Option<Document>,
    /// Message is a photo, available sizes of the photo.
    pub photo: Option<Vec<PhotoSize>>,
    /// Message is a sticker, information about the sticker.
    pub sticker: Option<Sticker>,
    /// Message is a video, information about the video.
    pub video: Option<Video>,
    /// Message is a voice message, information about the file.
    pub voice: Option<Voice>,
    /// Message is a video note message, information about the file.
    pub video_note: Option<VideoNote>,
    /// Caption for the document, photo or video, 0-200 characters.
    pub caption: Option<String>,
    /// Special entities that appear in the caption.
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// True, if the caption must be shown above the message media.
    pub show_caption_above_media: Option<bool>,
    /// True, if the message media is covered by a spoiler animation.
    pub has_media_spoiler: Option<bool>,
    /// Message is a shared contact, information about the contact.
    pub contact: Option<Contact>,
    /// Message is a dice with random value.
    pub dice: Option<super::dice::Dice>,
    /// Message is a shared location, information about the location.
    pub location: Option<Location>,
    /// Message is a native poll, information about the poll.
    pub poll: Option<Poll>,
    /// Message is a venue, information about the venue.
    pub venue: Option<Venue>,
    /// Message is a forwarded story.
    pub story: Option<super::story::Story>,
    /// New members that were added to the group or supergroup.
    pub new_chat_members: Option<Vec<User>>,
    /// A member was removed from the group.
    pub left_chat_member: Option<User>,
    /// A chat title was changed to this value.
    pub new_chat_title: Option<String>,
    /// A chat photo was changed to this value.
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    /// Service message: the chat photo was deleted.
    pub delete_chat_photo: Option<True>,
    /// Service message: the group has been created.
    pub group_chat_created: Option<True>,
    /// Service message: the supergroup has been created.
    pub supergroup_chat_created: Option<True>,
    /// Service message: the channel has been created.
    pub channel_chat_created: Option<True>,
    /// Service message: auto-delete timer settings changed.
    pub message_auto_delete_timer_changed: Option<super::message_auto_delete::MessageAutoDeleteTimerChanged>,
    /// The group has been migrated to a supergroup with the specified identifier.
    pub migrate_to_chat_id: Option<Integer>,
    /// The supergroup has been migrated from a group with the specified identifier.
    pub migrate_from_chat_id: Option<Integer>,
    /// Specified message was pinned.
    pub pinned_message: Option<Box<MessageOrChannelPost>>,
    /// Service message: a user in the chat triggered another user’s proximity alert.
    pub proximity_alert_triggered: Option<super::proximity_alert::ProximityAlertTriggered>,
    /// Service message: forum topic created.
    pub forum_topic_created: Option<super::forum_topic::ForumTopicCreated>,
    /// Service message: forum topic edited.
    pub forum_topic_edited: Option<super::forum_topic::ForumTopicEdited>,
    /// Service message: forum topic closed.
    pub forum_topic_closed: Option<super::forum_topic::ForumTopicClosed>,
    /// Service message: forum topic reopened.
    pub forum_topic_reopened: Option<super::forum_topic::ForumTopicReopened>,
    /// Service message: General forum topic hidden.
    pub general_forum_topic_hidden: Option<super::forum_topic::GeneralForumTopicHidden>,
    /// Service message: General forum topic unhidden.
    pub general_forum_topic_unhidden: Option<super::forum_topic::GeneralForumTopicUnhidden>,
    /// Service message: a scheduled giveaway was created.
    pub giveaway_created: Option<super::giveaway::GiveawayCreated>,
    /// Message is a scheduled giveaway.
    pub giveaway: Option<super::giveaway::Giveaway>,
    /// A giveaway with public winners was completed.
    pub giveaway_winners: Option<super::giveaway::GiveawayWinners>,
    /// Service message: a giveaway without public winners was completed.
    pub giveaway_completed: Option<super::giveaway::GiveawayCompleted>,
    /// Service message: video chat scheduled.
    pub video_chat_scheduled: Option<super::video_chat::VideoChatScheduled>,
    /// Service message: video chat started.
    pub video_chat_started: Option<super::video_chat::VideoChatStarted>,
    /// Service message: video chat ended.
    pub video_chat_ended: Option<super::video_chat::VideoChatEnded>,
    /// Service message: new participants invited to a video chat.
    pub video_chat_participants_invited: Option<super::video_chat::VideoChatParticipantsInvited>,
    /// Service message: the user allowed the bot to write messages.
    pub write_access_allowed: Option<super::write_access_allowed::WriteAccessAllowed>,
    /// Service message: users were shared with the bot.
    pub users_shared: Option<super::users_shared::UsersShared>,
    /// Service message: a chat was shared with the bot.
    pub chat_shared: Option<super::users_shared::ChatShared>,
    /// Service message: data sent by a Web App.
    pub web_app_data: Option<super::web_app::WebAppData>,
    /// Message contains paid media.
    pub paid_media: Option<super::paid_media::PaidMediaInfo>,
    /// Service message: chat background was set.
    pub chat_background_set: Option<super::chat_background::ChatBackground>,
    /// The number of Telegram Stars that were paid by the sender to send the message.
    pub paid_star_count: Option<Integer>,
    /// If the sender of the message boosted the chat, the number of boosts added.
    pub boost_added: Option<Integer>,
    /// Message is an invoice for a payment.
    pub invoice: Option<super::successful_payment::Invoice>,
    /// Message is a service message about a successful payment.
    pub successful_payment: Option<super::successful_payment::SuccessfulPayment>,
    /// Message is a service message about a refunded payment.
    pub refunded_payment: Option<super::successful_payment::RefundedPayment>,
    /// The message is a service message about a gift sent or received.
    pub gift: Option<super::unique_gift::GiftInfo>,
    /// The message is a service message about a unique gift sent or received.
    pub unique_gift: Option<super::unique_gift::UniqueGiftInfo>,
    /// Service message: a user in the chat was granted premium subscription by another user.
    pub gift_upgrade_sent: Option<super::unique_gift::GiftInfo>,
    /// Service message: the price for paid messages has changed.
    pub paid_message_price_changed: Option<super::successful_payment::PaidMessagePriceChanged>,
    /// Message is a checklist.
    pub checklist: Option<super::checklist::Checklist>,
    /// Service message: checklist tasks were completed.
    pub checklist_tasks_done: Option<super::checklist::ChecklistTasksDone>,
    /// Service message: checklist tasks were added.
    pub checklist_tasks_added: Option<super::checklist::ChecklistTasksAdded>,
    /// Service message: a suggested post was approved.
    pub suggested_post_approved: Option<super::suggested_post::SuggestedPostApproved>,
    /// Service message: a suggested post approval failed.
    pub suggested_post_approval_failed: Option<super::suggested_post::SuggestedPostApprovalFailed>,
    /// Service message: a suggested post was declined.
    pub suggested_post_declined: Option<super::suggested_post::SuggestedPostDeclined>,
    /// Service message: a suggested post was paid.
    pub suggested_post_paid: Option<super::suggested_post::SuggestedPostPaid>,
    /// Service message: a suggested post was refunded.
    pub suggested_post_refunded: Option<super::suggested_post::SuggestedPostRefunded>,
    /// The domain name of the website on which the user has logged in.
    pub connected_website: Option<String>,
    /// Telegram Passport data.
    pub passport_data: Option<super::passport::PassportData>,
    /// Service message: the chat owner has left.
    pub chat_owner_left: Option<super::chat_owner::ChatOwnerLeft>,
    /// Service message: the chat owner has changed.
    pub chat_owner_changed: Option<super::chat_owner::ChatOwnerChanged>,
    /// Service message: the price for direct messages has changed.
    pub direct_message_price_changed: Option<super::direct_messages::DirectMessagePriceChanged>,
    /// Information about the suggested post.
    pub suggested_post_info: Option<super::suggested_post::SuggestedPostInfo>,
    /// The message was sent in a direct messages topic.
    pub direct_messages_topic: Option<super::direct_messages::DirectMessagesTopic>,
    /// The tag of the sender in the chat.
    pub sender_tag: Option<String>,
    /// True, if the message is a paid post.
    pub is_paid_post: Option<bool>,
    /// For replies to a checklist task, the identifier of the replied task.
    pub reply_to_checklist_task_id: Option<Integer>,
    /// Inline keyboard attached to the message.
    pub reply_markup: Option<super::reply_markup::InlineKeyboardMarkup>,
}

/// This object represents one special entity in a text message.
/// For example, hashtags, usernames, URLs, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct MessageEntity {
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: Integer,
    /// Length of the entity in UTF-16 code units
    pub length: Integer,
    /// Kind of the entity.
    pub kind: MessageEntityKind,
}

/// Kind of the entity.
#[derive(Debug, Clone, PartialEq)]
pub enum MessageEntityKind {
    Mention,
    Hashtag,
    CashTag,
    BotCommand,
    Url,
    Email,
    PhoneNumber,
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Spoiler,
    Blockquote,
    ExpandableBlockquote,
    Code,
    Pre { language: Option<String> },
    TextLink(String), // TODO(knsd) URL?
    TextMention(User),
    CustomEmoji(String),
    #[doc(hidden)]
    Unknown(RawMessageEntity),
}

impl Serialize for MessageEntity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use self::MessageEntityKind::*;

        let mut raw = RawMessageEntity {
            type_: match &self.kind {
                Mention => "mention".to_string(),
                Hashtag => "hashtag".to_string(),
                CashTag => "cashtag".to_string(),
                BotCommand => "bot_command".to_string(),
                Url => "url".to_string(),
                Email => "email".to_string(),
                PhoneNumber => "phone_number".to_string(),
                Bold => "bold".to_string(),
                Italic => "italic".to_string(),
                Underline => "underline".to_string(),
                Strikethrough => "strikethrough".to_string(),
                Spoiler => "spoiler".to_string(),
                Blockquote => "blockquote".to_string(),
                ExpandableBlockquote => "expandable_blockquote".to_string(),
                Code => "code".to_string(),
                Pre { .. } => "pre".to_string(),
                TextLink(_) => "text_link".to_string(),
                TextMention(_) => "text_mention".to_string(),
                CustomEmoji(_) => "custom_emoji".to_string(),
                Unknown(raw) => return raw.serialize(serializer),
            },
            offset: self.offset,
            length: self.length,
            url: None,
            user: None,
            language: None,
            custom_emoji_id: None,
        };

        match &self.kind {
            Pre { language } => raw.language = language.clone(),
            TextLink(url) => raw.url = Some(url.clone()),
            TextMention(user) => raw.user = Some(user.clone()),
            CustomEmoji(id) => raw.custom_emoji_id = Some(id.clone()),
            _ => {}
        }

        raw.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MessageEntity {
    fn deserialize<D>(deserializer: D) -> Result<MessageEntity, D::Error>
    where
        D: Deserializer<'de>,
    {
        use self::MessageEntityKind::*;

        let raw: RawMessageEntity = Deserialize::deserialize(deserializer)?;

        let offset = raw.offset;
        let length = raw.length;

        macro_rules! required_field {
            ($name:ident) => {{
                match raw.$name {
                    Some(val) => val,
                    None => return Err(D::Error::missing_field(stringify!($name))),
                }
            }};
        }

        let kind = match raw.type_.as_str() {
            "mention" => Mention,
            "hashtag" => Hashtag,
            "cashtag" => CashTag,
            "bot_command" => BotCommand,
            "url" => Url,
            "email" => Email,
            "phone_number" => PhoneNumber,
            "bold" => Bold,
            "italic" => Italic,
            "underline" => Underline,
            "strikethrough" => Strikethrough,
            "spoiler" => Spoiler,
            "blockquote" => Blockquote,
            "expandable_blockquote" => ExpandableBlockquote,
            "code" => Code,
            "pre" => Pre { language: raw.language },
            "text_link" => TextLink(required_field!(url)),
            "text_mention" => TextMention(required_field!(user)),
            "custom_emoji" => CustomEmoji(required_field!(custom_emoji_id)),
            _ => Unknown(raw),
        };

        Ok(MessageEntity {
            offset: offset,
            length: length,
            kind: kind,
        })
    }
}

/// This object represents one special entity in a text message.
/// For example, hashtags, usernames, URLs, etc. Directly mapped.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawMessageEntity {
    /// Type of the entity. Can be mention (@username), hashtag, bot_command, url, email,
    /// bold (bold text), italic (italic text), code (monowidth string), pre (monowidth block),
    /// text_link (for clickable text URLs), text_mention (for users without usernames).
    #[serde(rename = "type")]
    pub type_: String,
    /// Offset in UTF-16 code units to the start of the entity.
    pub offset: Integer,
    /// Length of the entity in UTF-16 code units.
    pub length: Integer,
    /// For “text_link” only, url that will be opened after user taps on the text.
    pub url: Option<String>,
    /// For “text_mention” only, the mentioned user.
    pub user: Option<User>,
    /// For “pre” only, the programming language of the entity text.
    pub language: Option<String>,
    /// For “custom_emoji” only, unique identifier of the custom emoji.
    pub custom_emoji_id: Option<String>,
}

/// This object represents one size of a photo or a file / sticker thumbnail.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PhotoSize {
    /// Unique identifier for this file.
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots.
    pub file_unique_id: String,
    /// Photo width.
    pub width: Integer,
    /// Photo height.
    pub height: Integer,
    /// File size.
    pub file_size: Option<Integer>,
}

/// This object represents an audio file to be treated as music by the Telegram clients.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Audio {
    /// Unique identifier for this file.
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots.
    pub file_unique_id: String,
    /// Duration of the audio in seconds as defined by sender.
    pub duration: Integer,
    /// Performer of the audio as defined by sender or by audio tags.
    pub performer: Option<String>,
    /// Title of the audio as defined by sender or by audio tags.
    pub title: Option<String>,
    /// Thumbnail of the album cover to which the music file belongs.
    pub thumbnail: Option<PhotoSize>,
    /// Original filename as defined by sender.
    pub file_name: Option<String>,
    /// MIME type of the file as defined by sender.
    pub mime_type: Option<String>,
    /// File size.
    pub file_size: Option<Integer>,
}

/// This object represents a general file (as opposed to photos, voice messages and audio files).
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Document {
    /// Unique file identifier.
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots.
    pub file_unique_id: String,
    /// Document thumbnail as defined by sender.
    pub thumbnail: Option<PhotoSize>,
    /// Original filename as defined by sender.
    pub file_name: Option<String>,
    /// MIME type of the file as defined by sender.
    pub mime_type: Option<String>,
    /// File size.
    pub file_size: Option<Integer>,
}

/// This object represents a sticker.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Sticker {
    /// Identifier for this file, which can be used to download or reuse the file.
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots.
    pub file_unique_id: String,
    /// Type of the sticker. Currently one of "regular", "mask", "custom_emoji".
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Sticker width.
    pub width: Integer,
    /// Sticker height.
    pub height: Integer,
    /// True, if the sticker is animated.
    pub is_animated: Option<bool>,
    /// True, if the sticker is a video sticker.
    pub is_video: Option<bool>,
    /// Sticker thumbnail in .webp or .jpg format.
    pub thumbnail: Option<PhotoSize>,
    /// Emoji associated with the sticker.
    pub emoji: Option<String>,
    /// The name of the sticker set this sticker belongs to.
    pub set_name: Option<String>,
    /// For custom emoji stickers, unique identifier of the custom emoji.
    pub custom_emoji_id: Option<String>,
    /// True, if the sticker must be repainted to a text color in messages.
    pub needs_repainting: Option<bool>,
    /// File size.
    pub file_size: Option<Integer>,
}

/// This object represents a video file.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Video {
    /// Unique identifier for this file.
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots.
    pub file_unique_id: String,
    /// Video width as defined by sender.
    pub width: Integer,
    /// Video height as defined by sender.
    pub height: Integer,
    /// Duration of the video in seconds as defined by sender.
    pub duration: Integer,
    /// Video thumbnail.
    pub thumbnail: Option<PhotoSize>,
    /// Original filename as defined by sender.
    pub file_name: Option<String>,
    /// Mime type of a file as defined by sender.
    pub mime_type: Option<String>,
    /// File size.
    pub file_size: Option<Integer>,
}

/// This object represents a voice note.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Voice {
    /// Unique identifier for this file.
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots.
    pub file_unique_id: String,
    /// Duration of the audio in seconds as defined by sender.
    pub duration: Integer,
    /// MIME type of the file as defined by sender.
    pub mime_type: Option<String>,
    /// File size.
    pub file_size: Option<Integer>,
}

/// This object represents a video message (available in Telegram apps as of v.4.0).
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct VideoNote {
    /// Unique identifier for this file.
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots.
    pub file_unique_id: String,
    pub length: Integer,
    /// Duration of the video in seconds as defined by sender.
    pub duration: Integer,
    /// Video thumbnail.
    pub thumbnail: Option<PhotoSize>,
    /// File size.
    pub file_size: Option<Integer>,
}

/// This object represents a phone contact.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Contact {
    /// Contact's phone number.
    pub phone_number: String,
    /// Contact's first name.
    pub first_name: String,
    /// Contact's last name.
    pub last_name: Option<String>,
    /// Contact's user identifier in Telegram.
    pub user_id: Option<Integer>,
    /// Additional data about the contact in the form of a vCard.
    pub vcard: Option<String>,
}

/// This object represents a point on the map.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Location {
    /// Longitude as defined by sender.
    pub longitude: Float,
    /// Latitude as defined by sender.
    pub latitude: Float,
    /// The radius of uncertainty for the location, measured in meters.
    pub horizontal_accuracy: Option<Float>,
    /// Time relative to the message sending date, during which the location can be updated.
    pub live_period: Option<Integer>,
    /// The direction in which user is moving, in degrees; 1-360.
    pub heading: Option<Integer>,
    /// The maximum distance for proximity alerts about approaching another chat member, in meters.
    pub proximity_alert_radius: Option<Integer>,
}

/// This object represents a venue.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Venue {
    /// Venue location.
    pub location: Location,
    /// Name of the venue.
    pub title: String,
    /// Address of the venue.
    pub address: String,
    /// Foursquare identifier of the venue.
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue.
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue.
    pub google_place_id: Option<String>,
    /// Google Places type of the venue.
    pub google_place_type: Option<String>,
}

/// This object contains information about a poll.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Poll {
    /// Unique poll identifier.
    pub id: String,
    /// Poll question.
    pub question: String,
    /// Special entities that appear in the question.
    pub question_entities: Option<Vec<MessageEntity>>,
    /// List of poll options.
    pub options: Vec<PollOption>,
    /// Total number of users that voted in the poll.
    pub total_voter_count: Integer,
    /// True, if the poll is closed.
    pub is_closed: bool,
    /// True, if the poll is anonymous.
    pub is_anonymous: bool,
    /// Poll type.
    #[serde(rename = "type")]
    pub type_: PollType,
    /// True, if the poll allows multiple answers.
    pub allows_multiple_answers: bool,
    /// 0-based identifier of the correct answer option. Available only for polls in the quiz mode,
    /// which are closed, or was sent (not forwarded) by the bot or to the private chat with the bot.
    pub correct_option_id: Option<Integer>,
    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll.
    pub explanation: Option<String>,
    /// Special entities like usernames, URLs, bot commands, etc. that appear in the explanation.
    pub explanation_entities: Option<Vec<MessageEntity>>,
    /// Amount of time in seconds the poll will be active after creation.
    pub open_period: Option<Integer>,
    /// Point in time (Unix timestamp) when the poll will be automatically closed.
    pub close_date: Option<Integer>,
}

/// This object represents an answer of a user in a non-anonymous poll.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PollAnswer {
    /// Unique poll identifier.
    pub poll_id: String,
    /// The chat that changed the answer to the poll, if the voter is anonymous.
    pub voter_chat: Option<Chat>,
    /// The user that changed the answer to the poll, if the voter isn't anonymous.
    pub user: Option<User>,
    /// 0-based identifiers of answer options, chosen by the user. May be empty if the user retracted their vote.
    pub option_ids: Vec<Integer>,
}

/// This object contains information about one answer option in a poll.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PollOption {
    /// Option text.
    pub text: String,
    /// Special entities that appear in the option text.
    pub text_entities: Option<Vec<MessageEntity>>,
    /// Number of users that voted for this option.
    pub voter_count: Integer,
}

/// Type of a poll.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PollType {
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "quiz")]
    Quiz,
}

/// This object represent a user's profile pictures.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has.
    pub total_count: Integer,
    /// Requested profile pictures (in up to 4 sizes each).
    pub photos: Vec<Vec<PhotoSize>>,
}

/// This object represents a file ready to be downloaded.
/// The file can be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`.
/// It is guaranteed that the link will be valid for at least 1 hour.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct File {
    /// Unique identifier for this file.
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots.
    pub file_unique_id: String,
    /// File size, if known.
    pub file_size: Option<Integer>,
    /// File path. Use `https://api.telegram.org/file/bot<token>/<file_path>` to get the file.
    pub file_path: Option<String>,
}

impl File {
    pub fn get_url(&self, token: &str) -> Option<String> {
        self.file_path
            .as_ref()
            .map(|path| format!("{}file/bot{}/{}", telegram_api_url(), token, path))
    }
}

/// Strongly typed ParseMode.
/// See [documentation](https://core.telegram.org/bots/api#formatting-options) for details.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum ParseMode {
    /// Use legacy markdown formatting.
    Markdown,
    /// Use MarkdownV2 formatting.
    MarkdownV2,
    /// Use HTML formatting.
    #[serde(rename = "HTML")]
    Html,
}

impl ::std::fmt::Display for ParseMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ParseMode::Markdown => write!(f, "Markdown"),
            ParseMode::MarkdownV2 => write!(f, "MarkdownV2"),
            ParseMode::Html => write!(f, "HTML"),
        }
    }
}
