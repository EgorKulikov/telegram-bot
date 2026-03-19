use crate::types::*;

/// This object represents a bot command.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct BotCommand {
    pub command: String,
    pub description: String,
}

/// This object represents the scope to which bot commands are applied.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum BotCommandScope {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "all_private_chats")]
    AllPrivateChats,
    #[serde(rename = "all_group_chats")]
    AllGroupChats,
    #[serde(rename = "all_chat_administrators")]
    AllChatAdministrators,
    #[serde(rename = "chat")]
    Chat { chat_id: ChatRef },
    #[serde(rename = "chat_administrators")]
    ChatAdministrators { chat_id: ChatRef },
    #[serde(rename = "chat_member")]
    ChatMember { chat_id: ChatRef, user_id: UserId },
}

/// This object represents the bot's name.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct BotName {
    pub name: String,
}

/// This object represents the bot's description.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct BotDescription {
    pub description: String,
}

/// This object represents the bot's short description.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct BotShortDescription {
    pub short_description: String,
}
