use crate::types::*;

/// Describes the connection of the bot with a business account.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct BusinessConnection {
    pub id: String,
    pub user: User,
    pub user_chat_id: Integer,
    pub date: Integer,
    pub can_reply: Option<bool>,
    pub is_enabled: bool,
    pub rights: Option<BusinessBotRights>,
}

/// Represents the rights of a bot in a business account.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct BusinessBotRights {
    pub can_reply: Option<bool>,
    pub can_read_messages: Option<bool>,
    pub can_delete_outgoing_messages: Option<bool>,
    pub can_delete_all_messages: Option<bool>,
    pub can_edit_name: Option<bool>,
    pub can_edit_bio: Option<bool>,
    pub can_edit_profile_photo: Option<bool>,
    pub can_edit_username: Option<bool>,
    pub can_change_gift_settings: Option<bool>,
    pub can_view_gifts_and_stars: Option<bool>,
    pub can_convert_gifts_to_stars: Option<bool>,
    pub can_transfer_and_upgrade_gifts: Option<bool>,
    pub can_transfer_stars: Option<bool>,
    pub can_manage_stories: Option<bool>,
}

/// This object is received when messages are deleted from a connected business account.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct BusinessMessagesDeleted {
    pub business_connection_id: String,
    pub chat: Chat,
    pub message_ids: Vec<Integer>,
}

/// Describes the opening hours of a business.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct BusinessOpeningHours {
    pub time_zone_name: String,
    pub opening_hours: Vec<BusinessOpeningHoursInterval>,
}

/// Describes an interval of time during which a business is open.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct BusinessOpeningHoursInterval {
    pub opening_minute: Integer,
    pub closing_minute: Integer,
}

/// Describes the location of a business.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct BusinessLocation {
    pub address: String,
    pub location: Option<Location>,
}

/// Describes the intro of a business.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct BusinessIntro {
    pub title: Option<String>,
    pub message: Option<String>,
    pub sticker: Option<Sticker>,
}
