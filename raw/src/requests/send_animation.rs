use std::borrow::Cow;

use crate::requests::*;
use crate::types::*;

/// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound).
#[derive(Debug, Clone, PartialEq)]
#[must_use = "requests do nothing unless sent"]
pub struct SendAnimation<'c> {
    chat_id: ChatRef,
    animation: InputFile,
    caption: Option<Cow<'c, str>>,
    parse_mode: Option<ParseMode>,
    reply_to_message_id: Option<MessageId>,
    disable_notification: bool,
    reply_markup: Option<ReplyMarkup>,
}

impl<'c> ToMultipart for SendAnimation<'c> {
    fn to_multipart(&self) -> Result<Multipart, Error> {
        multipart_map! {
            self,
            (chat_id (text));
            (animation (raw));
            (caption (text), optional);
            (parse_mode (text), optional);
            (reply_to_message_id (text), optional);
            (disable_notification (text), when_true);
            (reply_markup (json), optional);
        }
    }
}

impl<'c> Request for SendAnimation<'c> {
    type Type = MultipartRequestType<Self>;
    type Response = JsonIdResponse<Message>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("sendAnimation"), self)
    }
}

impl<'c> SendAnimation<'c> {
    pub fn new<C, V>(chat: C, animation: V) -> Self
    where
        C: ToChatRef,
        V: Into<InputFile>,
    {
        Self {
            chat_id: chat.to_chat_ref(),
            animation: animation.into(),
            caption: None,
            parse_mode: None,
            reply_to_message_id: None,
            reply_markup: None,
            disable_notification: false,
        }
    }

    pub fn caption<T>(&mut self, caption: T) -> &mut Self
    where
        T: Into<Cow<'c, str>>,
    {
        self.caption = Some(caption.into());
        self
    }

    pub fn parse_mode(&mut self, parse_mode: ParseMode) -> &mut Self {
        self.parse_mode = Some(parse_mode);
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

    pub fn reply_markup<R: Into<ReplyMarkup>>(&mut self, reply_markup: R) -> &mut Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}
