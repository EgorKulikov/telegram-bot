use crate::types::*;

/// This object contains information about one member of a chat.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "status")]
pub enum ChatMember {
    #[serde(rename = "creator")]
    Owner {
        user: User,
        #[serde(default)]
        is_anonymous: bool,
        custom_title: Option<String>,
    },
    #[serde(rename = "administrator")]
    Administrator {
        user: User,
        #[serde(default)]
        can_be_edited: bool,
        #[serde(default)]
        is_anonymous: bool,
        #[serde(default)]
        can_manage_chat: bool,
        #[serde(default)]
        can_delete_messages: bool,
        #[serde(default)]
        can_manage_video_chats: bool,
        #[serde(default)]
        can_restrict_members: bool,
        #[serde(default)]
        can_promote_members: bool,
        #[serde(default)]
        can_change_info: bool,
        #[serde(default)]
        can_invite_users: bool,
        #[serde(default)]
        can_post_stories: bool,
        #[serde(default)]
        can_edit_stories: bool,
        #[serde(default)]
        can_delete_stories: bool,
        #[serde(default)]
        can_post_messages: bool,
        #[serde(default)]
        can_edit_messages: bool,
        #[serde(default)]
        can_pin_messages: bool,
        #[serde(default)]
        can_manage_topics: bool,
        custom_title: Option<String>,
    },
    #[serde(rename = "member")]
    Member {
        user: User,
        until_date: Option<Integer>,
    },
    #[serde(rename = "restricted")]
    Restricted {
        user: User,
        #[serde(default)]
        is_member: bool,
        #[serde(default)]
        can_send_messages: bool,
        #[serde(default)]
        can_send_audios: bool,
        #[serde(default)]
        can_send_documents: bool,
        #[serde(default)]
        can_send_photos: bool,
        #[serde(default)]
        can_send_videos: bool,
        #[serde(default)]
        can_send_video_notes: bool,
        #[serde(default)]
        can_send_voice_notes: bool,
        #[serde(default)]
        can_send_polls: bool,
        #[serde(default)]
        can_send_other_messages: bool,
        #[serde(default)]
        can_add_web_page_previews: bool,
        #[serde(default)]
        can_change_info: bool,
        #[serde(default)]
        can_invite_users: bool,
        #[serde(default)]
        can_pin_messages: bool,
        #[serde(default)]
        can_manage_topics: bool,
        until_date: Option<Integer>,
    },
    #[serde(rename = "left")]
    Left {
        user: User,
    },
    #[serde(rename = "kicked")]
    Banned {
        user: User,
        until_date: Option<Integer>,
    },
}

impl ChatMember {
    /// Get the user from any ChatMember variant.
    pub fn user(&self) -> &User {
        match self {
            ChatMember::Owner { user, .. } => user,
            ChatMember::Administrator { user, .. } => user,
            ChatMember::Member { user, .. } => user,
            ChatMember::Restricted { user, .. } => user,
            ChatMember::Left { user, .. } => user,
            ChatMember::Banned { user, .. } => user,
        }
    }
}
