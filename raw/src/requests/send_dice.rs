use crate::requests::*;
use crate::types::*;

/// Use this method to send an animated emoji that will display a random value.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SendDice {
    chat_id: ChatRef,
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl Request for SendDice {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<MessageOrChannelPost>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("sendDice"), self)
    }
}

impl SendDice {
    pub fn new<C: ToChatRef>(chat: C) -> Self {
        SendDice {
            chat_id: chat.to_chat_ref(),
            emoji: None,
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
            reply_markup: None,
        }
    }

    pub fn emoji<S: Into<String>>(&mut self, emoji: S) -> &mut Self {
        self.emoji = Some(emoji.into());
        self
    }
}
