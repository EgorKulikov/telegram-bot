use crate::types::web_app::WebAppInfo;

/// This object describes the bot's menu button in a private chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MenuButton {
    #[serde(rename = "commands")]
    Commands,
    #[serde(rename = "web_app")]
    WebApp {
        text: String,
        web_app: WebAppInfo,
    },
    #[serde(rename = "default")]
    Default,
}
