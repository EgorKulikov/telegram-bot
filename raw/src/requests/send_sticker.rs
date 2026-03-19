use crate::requests::*;
use crate::types::*;

/// Use this method to send static .WEBP, animated .TGS, or video .WEBM stickers.
#[derive(Debug, Clone, PartialEq)]
#[must_use = "requests do nothing unless sent"]
pub struct SendSticker {
    chat_id: ChatRef,
    sticker: InputFile,
    emoji: Option<String>,
    disable_notification: bool,
    protect_content: Option<bool>,
    reply_to_message_id: Option<MessageId>,
    reply_markup: Option<ReplyMarkup>,
}

impl ToMultipart for SendSticker {
    fn to_multipart(&self) -> Result<Multipart, Error> {
        multipart_map! {
            self,
            (chat_id (text));
            (sticker (raw));
            (emoji (text), optional);
            (disable_notification (text), when_true);
            (protect_content (text), optional);
            (reply_to_message_id (text), optional);
            (reply_markup (json), optional);
        }
    }
}

impl Request for SendSticker {
    type Type = MultipartRequestType<Self>;
    type Response = JsonIdResponse<Message>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("sendSticker"), self)
    }
}

impl SendSticker {
    pub fn new<C: ToChatRef, V: Into<InputFile>>(chat: C, sticker: V) -> Self {
        Self {
            chat_id: chat.to_chat_ref(),
            sticker: sticker.into(),
            emoji: None,
            disable_notification: false,
            protect_content: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn emoji<S: Into<String>>(&mut self, emoji: S) -> &mut Self {
        self.emoji = Some(emoji.into());
        self
    }

    pub fn reply_to<R: ToMessageId>(&mut self, to: R) -> &mut Self {
        self.reply_to_message_id = Some(to.to_message_id());
        self
    }

    pub fn disable_notification(&mut self) -> &mut Self {
        self.disable_notification = true;
        self
    }
}
