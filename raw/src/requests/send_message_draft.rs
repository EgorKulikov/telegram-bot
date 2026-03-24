use std::borrow::Cow;

use crate::requests::*;
use crate::types::*;

/// Use this method to stream a partial message to a user while the message is being generated.
/// Returns True on success.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SendMessageDraft<'s> {
    chat_id: ChatRef,
    draft_id: Integer,
    text: Cow<'s, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<Vec<MessageEntity>>,
}

impl<'s> Request for SendMessageDraft<'s> {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("sendMessageDraft"), self)
    }
}

impl<'s> SendMessageDraft<'s> {
    pub fn new<C, T>(chat: C, draft_id: Integer, text: T) -> Self
    where
        C: ToChatRef,
        T: Into<Cow<'s, str>>,
    {
        SendMessageDraft {
            chat_id: chat.to_chat_ref(),
            draft_id,
            text: text.into(),
            message_thread_id: None,
            parse_mode: None,
            entities: None,
        }
    }

    pub fn message_thread_id(&mut self, message_thread_id: Integer) -> &mut Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }

    pub fn parse_mode(&mut self, parse_mode: ParseMode) -> &mut Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    pub fn entities(&mut self, entities: Vec<MessageEntity>) -> &mut Self {
        self.entities = Some(entities);
        self
    }
}

/// Send a message draft to a chat.
pub trait CanSendMessageDraft {
    fn message_draft<'s, T>(&self, draft_id: Integer, text: T) -> SendMessageDraft<'s>
    where
        T: Into<Cow<'s, str>>;
}

impl<C> CanSendMessageDraft for C
where
    C: ToChatRef,
{
    fn message_draft<'s, T>(&self, draft_id: Integer, text: T) -> SendMessageDraft<'s>
    where
        T: Into<Cow<'s, str>>,
    {
        SendMessageDraft::new(self, draft_id, text)
    }
}
