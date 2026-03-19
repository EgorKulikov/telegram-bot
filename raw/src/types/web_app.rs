/// Describes a Web App.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct WebAppInfo {
    pub url: String,
}

/// Describes data sent from a Web App to the bot.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct WebAppData {
    pub data: String,
    pub button_text: String,
}

/// Describes an inline message sent by a Web App on behalf of a user.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct SentWebAppMessage {
    pub inline_message_id: Option<String>,
}
