use crate::requests::*;
use crate::types::*;

/// Use this method to send video messages (round video).
#[derive(Debug, Clone, PartialEq)]
#[must_use = "requests do nothing unless sent"]
pub struct SendVideoNote {
    chat_id: ChatRef,
    video_note: InputFile,
    duration: Option<Integer>,
    length: Option<Integer>,
    disable_notification: bool,
    reply_to_message_id: Option<MessageId>,
    reply_markup: Option<ReplyMarkup>,
}

impl ToMultipart for SendVideoNote {
    fn to_multipart(&self) -> Result<Multipart, Error> {
        multipart_map! {
            self,
            (chat_id (text));
            (video_note (raw));
            (duration (text), optional);
            (length (text), optional);
            (disable_notification (text), when_true);
            (reply_to_message_id (text), optional);
            (reply_markup (json), optional);
        }
    }
}

impl Request for SendVideoNote {
    type Type = MultipartRequestType<Self>;
    type Response = JsonIdResponse<Message>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("sendVideoNote"), self)
    }
}

impl SendVideoNote {
    pub fn new<C: ToChatRef, V: Into<InputFile>>(chat: C, video_note: V) -> Self {
        Self {
            chat_id: chat.to_chat_ref(),
            video_note: video_note.into(),
            duration: None,
            length: None,
            disable_notification: false,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn duration(&mut self, duration: Integer) -> &mut Self {
        self.duration = Some(duration);
        self
    }

    pub fn length(&mut self, length: Integer) -> &mut Self {
        self.length = Some(length);
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
