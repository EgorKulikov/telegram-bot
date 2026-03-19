use crate::requests::*;
use crate::types::*;

/// Use this method to change the chosen reactions on a message.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SetMessageReaction {
    chat_id: ChatRef,
    message_id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    reaction: Option<Vec<ReactionType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_big: Option<bool>,
}

impl Request for SetMessageReaction {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("setMessageReaction"), self)
    }
}

impl SetMessageReaction {
    pub fn new<C, M>(chat: C, message: M) -> Self
    where
        C: ToChatRef,
        M: ToMessageId,
    {
        SetMessageReaction {
            chat_id: chat.to_chat_ref(),
            message_id: message.to_message_id(),
            reaction: None,
            is_big: None,
        }
    }

    pub fn reaction(&mut self, reaction: Vec<ReactionType>) -> &mut Self {
        self.reaction = Some(reaction);
        self
    }

    pub fn is_big(&mut self, is_big: bool) -> &mut Self {
        self.is_big = Some(is_big);
        self
    }
}
