use crate::requests::*;
use crate::types::*;

#[derive(Debug, Clone, PartialEq)]
#[must_use = "requests do nothing unless sent"]
pub struct SetChatPhoto {
    chat_id: ChatRef,
    photo: InputFile,
}

impl ToMultipart for SetChatPhoto {
    fn to_multipart(&self) -> Result<Multipart, Error> {
        multipart_map! { self, (chat_id (text)); (photo (raw)); }
    }
}

impl Request for SetChatPhoto {
    type Type = MultipartRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("setChatPhoto"), self)
    }
}

impl SetChatPhoto {
    pub fn new<C: ToChatRef, V: Into<InputFile>>(chat: C, photo: V) -> Self {
        SetChatPhoto { chat_id: chat.to_chat_ref(), photo: photo.into() }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct DeleteChatPhoto { chat_id: ChatRef }

impl Request for DeleteChatPhoto {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("deleteChatPhoto"), self)
    }
}

impl DeleteChatPhoto {
    pub fn new<C: ToChatRef>(chat: C) -> Self { DeleteChatPhoto { chat_id: chat.to_chat_ref() } }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SetChatTitle { chat_id: ChatRef, title: String }

impl Request for SetChatTitle {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("setChatTitle"), self)
    }
}

impl SetChatTitle {
    pub fn new<C: ToChatRef, S: Into<String>>(chat: C, title: S) -> Self {
        SetChatTitle { chat_id: chat.to_chat_ref(), title: title.into() }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SetChatDescription {
    chat_id: ChatRef,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

impl Request for SetChatDescription {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("setChatDescription"), self)
    }
}

impl SetChatDescription {
    pub fn new<C: ToChatRef>(chat: C) -> Self {
        SetChatDescription { chat_id: chat.to_chat_ref(), description: None }
    }
    pub fn description<S: Into<String>>(&mut self, desc: S) -> &mut Self {
        self.description = Some(desc.into()); self
    }
}
