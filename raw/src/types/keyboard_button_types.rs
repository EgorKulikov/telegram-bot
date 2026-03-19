use crate::types::*;

/// This object defines the criteria used to request suitable users.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyboardButtonRequestUsers {
    pub request_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_premium: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_quantity: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_username: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_photo: Option<bool>,
}

/// This object defines the criteria used to request a suitable chat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyboardButtonRequestChat {
    pub request_id: Integer,
    pub chat_is_channel: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_forum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_has_username: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_created: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_administrator_rights: Option<super::chat_administrator_rights::ChatAdministratorRights>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_administrator_rights: Option<super::chat_administrator_rights::ChatAdministratorRights>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_is_member: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_title: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_username: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_photo: Option<bool>,
}

/// This object represents type of a poll, which is allowed to be created and sent when the corresponding button is pressed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyboardButtonPollType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
