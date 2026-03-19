use crate::requests::*;
use crate::types::*;

/// Use this method to change the bot's menu button in a private chat.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SetChatMenuButton {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    menu_button: Option<MenuButton>,
}

impl Request for SetChatMenuButton {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("setChatMenuButton"), self)
    }
}

impl SetChatMenuButton {
    pub fn new() -> Self {
        SetChatMenuButton {
            chat_id: None,
            menu_button: None,
        }
    }

    pub fn chat_id<C: ToChatRef>(&mut self, chat: C) -> &mut Self {
        self.chat_id = Some(chat.to_chat_ref());
        self
    }

    pub fn menu_button(&mut self, button: MenuButton) -> &mut Self {
        self.menu_button = Some(button);
        self
    }
}

/// Use this method to get the current value of the bot's menu button in a private chat.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetChatMenuButton {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatRef>,
}

impl Request for GetChatMenuButton {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<MenuButton>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("getChatMenuButton"), self)
    }
}

impl GetChatMenuButton {
    pub fn new() -> Self {
        GetChatMenuButton { chat_id: None }
    }
}
