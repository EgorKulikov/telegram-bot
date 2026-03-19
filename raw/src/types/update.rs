use crate::types::*;

/// This object represents an incoming update.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Update {
    /// The update's unique identifier. Update identifiers start from a certain
    /// positive number and increase sequentially.
    #[serde(rename = "update_id")]
    pub id: Integer,
    /// Kind of the incoming update.
    #[serde(flatten)]
    pub kind: UpdateKind,
}

/// Kind of the incoming update.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum UpdateKind {
    /// New incoming message of any kind — text, photo, sticker, etc.
    #[serde(rename = "message")]
    Message(Message),
    /// New version of a message that is known to the bot and was edited.
    #[serde(rename = "edited_message")]
    EditedMessage(Message),
    /// New incoming channel post of any kind — text, photo, sticker, etc.
    #[serde(rename = "channel_post")]
    ChannelPost(ChannelPost),
    /// New version of a channel post that is known to the bot and was edited.
    #[serde(rename = "edited_channel_post")]
    EditedChannelPost(ChannelPost),
    /// New incoming business message.
    #[serde(rename = "business_message")]
    BusinessMessage(Message),
    /// New version of a business message that is known to the bot and was edited.
    #[serde(rename = "edited_business_message")]
    EditedBusinessMessage(Message),
    /// Messages were deleted from a connected business account.
    #[serde(rename = "deleted_business_messages")]
    DeletedBusinessMessages(super::business::BusinessMessagesDeleted),
    /// The bot was connected to or disconnected from a business account.
    #[serde(rename = "business_connection")]
    BusinessConnection(super::business::BusinessConnection),
    /// A reaction to a message was changed by a user.
    #[serde(rename = "message_reaction")]
    MessageReaction(super::reaction_type::MessageReactionUpdated),
    /// Reactions to a message with anonymous reactions were changed.
    #[serde(rename = "message_reaction_count")]
    MessageReactionCount(super::reaction_type::MessageReactionCountUpdated),
    #[serde(rename = "inline_query")]
    InlineQuery(InlineQuery),
    /// The result of an inline query that was chosen by a user and sent to their chat partner.
    #[serde(rename = "chosen_inline_result")]
    ChosenInlineResult(ChosenInlineResult),
    #[serde(rename = "callback_query")]
    CallbackQuery(CallbackQuery),
    /// New incoming shipping query. Only for invoices with flexible price.
    #[serde(rename = "shipping_query")]
    ShippingQuery(ShippingQuery),
    /// New incoming pre-checkout query. Contains full information about checkout.
    #[serde(rename = "pre_checkout_query")]
    PreCheckoutQuery(PreCheckoutQuery),
    /// New poll state. Bots receive only updates about stopped polls and polls, which are sent by the bot.
    #[serde(rename = "poll")]
    Poll(Poll),
    /// A user changed their answer in a non-anonymous poll.
    #[serde(rename = "poll_answer")]
    PollAnswer(PollAnswer),
    /// The bot's chat member status was updated in a chat.
    #[serde(rename = "my_chat_member")]
    MyChatMember(ChatMemberUpdate),
    /// A chat member's status was updated in a chat.
    #[serde(rename = "chat_member")]
    ChatMember(ChatMemberUpdate),
    /// A request to join the chat has been sent.
    #[serde(rename = "chat_join_request")]
    ChatJoinRequest(ChatJoinRequest),
    /// A chat boost was added or changed.
    #[serde(rename = "chat_boost")]
    ChatBoostUpdated(ChatBoostUpdated),
    /// A boost was removed from a chat.
    #[serde(rename = "removed_chat_boost")]
    ChatBoostRemoved(ChatBoostRemoved),
    /// A user purchased paid media with a non-empty payload sent by the bot in a non-channel chat.
    #[serde(rename = "purchased_paid_media")]
    PurchasedPaidMedia(super::paid_media::PaidMediaPurchased),
    #[doc(hidden)]
    Error(String),
    #[doc(hidden)]
    Unknown,
}
