use std::borrow::Cow;

use crate::requests::*;
use crate::types::*;

/// Use this method to send audio files to be displayed as a playable voice message.
#[derive(Debug, Clone, PartialEq)]
#[must_use = "requests do nothing unless sent"]
pub struct SendVoice<'c> {
    chat_id: ChatRef,
    voice: InputFile,
    caption: Option<Cow<'c, str>>,
    parse_mode: Option<ParseMode>,
    duration: Option<Integer>,
    disable_notification: bool,
    reply_to_message_id: Option<MessageId>,
    reply_markup: Option<ReplyMarkup>,
}

impl<'c> ToMultipart for SendVoice<'c> {
    fn to_multipart(&self) -> Result<Multipart, Error> {
        multipart_map! {
            self,
            (chat_id (text));
            (voice (raw));
            (caption (text), optional);
            (parse_mode (text), optional);
            (duration (text), optional);
            (disable_notification (text), when_true);
            (reply_to_message_id (text), optional);
            (reply_markup (json), optional);
        }
    }
}

impl<'c> Request for SendVoice<'c> {
    type Type = MultipartRequestType<Self>;
    type Response = JsonIdResponse<Message>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("sendVoice"), self)
    }
}

impl<'c> SendVoice<'c> {
    pub fn new<C: ToChatRef, V: Into<InputFile>>(chat: C, voice: V) -> Self {
        Self {
            chat_id: chat.to_chat_ref(),
            voice: voice.into(),
            caption: None,
            parse_mode: None,
            duration: None,
            disable_notification: false,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn caption<T: Into<Cow<'c, str>>>(&mut self, caption: T) -> &mut Self {
        self.caption = Some(caption.into());
        self
    }

    pub fn parse_mode(&mut self, parse_mode: ParseMode) -> &mut Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    pub fn duration(&mut self, duration: Integer) -> &mut Self {
        self.duration = Some(duration);
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
