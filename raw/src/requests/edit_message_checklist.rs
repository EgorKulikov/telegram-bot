use crate::requests::*;
use crate::types::*;

/// Use this method to edit a checklist message sent by the bot.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct EditMessageChecklist {
    chat_id: ChatRef,
    message_id: MessageId,
    checklist: InputChecklist,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}
impl Request for EditMessageChecklist {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<MessageOrChannelPost>;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("editMessageChecklist"), self) }
}
impl EditMessageChecklist {
    pub fn new<C: ToChatRef, M: ToMessageId>(chat: C, message_id: M, checklist: InputChecklist) -> Self {
        EditMessageChecklist {
            chat_id: chat.to_chat_ref(),
            message_id: message_id.to_message_id(),
            checklist,
            reply_markup: None,
        }
    }
    pub fn reply_markup<R: Into<ReplyMarkup>>(&mut self, reply_markup: R) -> &mut Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}
