use crate::requests::*;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct CreateForumTopic {
    chat_id: ChatRef,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_color: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_custom_emoji_id: Option<String>,
}
impl Request for CreateForumTopic {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<ForumTopic>;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("createForumTopic"), self) }
}
impl CreateForumTopic {
    pub fn new<C: ToChatRef, S: Into<String>>(chat: C, name: S) -> Self {
        CreateForumTopic { chat_id: chat.to_chat_ref(), name: name.into(), icon_color: None, icon_custom_emoji_id: None }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct EditForumTopic {
    chat_id: ChatRef,
    message_thread_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_custom_emoji_id: Option<String>,
}
impl Request for EditForumTopic {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("editForumTopic"), self) }
}
impl EditForumTopic {
    pub fn new<C: ToChatRef>(chat: C, message_thread_id: Integer) -> Self {
        EditForumTopic { chat_id: chat.to_chat_ref(), message_thread_id, name: None, icon_custom_emoji_id: None }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct CloseForumTopic { chat_id: ChatRef, message_thread_id: Integer }
impl Request for CloseForumTopic {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("closeForumTopic"), self) }
}
impl CloseForumTopic {
    pub fn new<C: ToChatRef>(chat: C, message_thread_id: Integer) -> Self {
        CloseForumTopic { chat_id: chat.to_chat_ref(), message_thread_id }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct ReopenForumTopic { chat_id: ChatRef, message_thread_id: Integer }
impl Request for ReopenForumTopic {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("reopenForumTopic"), self) }
}
impl ReopenForumTopic {
    pub fn new<C: ToChatRef>(chat: C, message_thread_id: Integer) -> Self {
        ReopenForumTopic { chat_id: chat.to_chat_ref(), message_thread_id }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct DeleteForumTopic { chat_id: ChatRef, message_thread_id: Integer }
impl Request for DeleteForumTopic {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("deleteForumTopic"), self) }
}
impl DeleteForumTopic {
    pub fn new<C: ToChatRef>(chat: C, message_thread_id: Integer) -> Self {
        DeleteForumTopic { chat_id: chat.to_chat_ref(), message_thread_id }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct UnpinAllForumTopicMessages { chat_id: ChatRef, message_thread_id: Integer }
impl Request for UnpinAllForumTopicMessages {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("unpinAllForumTopicMessages"), self) }
}
impl UnpinAllForumTopicMessages {
    pub fn new<C: ToChatRef>(chat: C, message_thread_id: Integer) -> Self {
        UnpinAllForumTopicMessages { chat_id: chat.to_chat_ref(), message_thread_id }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct EditGeneralForumTopic { chat_id: ChatRef, name: String }
impl Request for EditGeneralForumTopic {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("editGeneralForumTopic"), self) }
}
impl EditGeneralForumTopic {
    pub fn new<C: ToChatRef, S: Into<String>>(chat: C, name: S) -> Self {
        EditGeneralForumTopic { chat_id: chat.to_chat_ref(), name: name.into() }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct CloseGeneralForumTopic { chat_id: ChatRef }
impl Request for CloseGeneralForumTopic {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("closeGeneralForumTopic"), self) }
}
impl CloseGeneralForumTopic {
    pub fn new<C: ToChatRef>(chat: C) -> Self { CloseGeneralForumTopic { chat_id: chat.to_chat_ref() } }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct ReopenGeneralForumTopic { chat_id: ChatRef }
impl Request for ReopenGeneralForumTopic {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("reopenGeneralForumTopic"), self) }
}
impl ReopenGeneralForumTopic {
    pub fn new<C: ToChatRef>(chat: C) -> Self { ReopenGeneralForumTopic { chat_id: chat.to_chat_ref() } }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct HideGeneralForumTopic { chat_id: ChatRef }
impl Request for HideGeneralForumTopic {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("hideGeneralForumTopic"), self) }
}
impl HideGeneralForumTopic {
    pub fn new<C: ToChatRef>(chat: C) -> Self { HideGeneralForumTopic { chat_id: chat.to_chat_ref() } }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct UnhideGeneralForumTopic { chat_id: ChatRef }
impl Request for UnhideGeneralForumTopic {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("unhideGeneralForumTopic"), self) }
}
impl UnhideGeneralForumTopic {
    pub fn new<C: ToChatRef>(chat: C) -> Self { UnhideGeneralForumTopic { chat_id: chat.to_chat_ref() } }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct UnpinAllGeneralForumTopicMessages { chat_id: ChatRef }
impl Request for UnpinAllGeneralForumTopicMessages {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("unpinAllGeneralForumTopicMessages"), self) }
}
impl UnpinAllGeneralForumTopicMessages {
    pub fn new<C: ToChatRef>(chat: C) -> Self { UnpinAllGeneralForumTopicMessages { chat_id: chat.to_chat_ref() } }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetForumTopicIconStickers {}
impl Request for GetForumTopicIconStickers {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<Vec<Sticker>>;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("getForumTopicIconStickers"), self) }
}
impl GetForumTopicIconStickers { pub fn new() -> Self { GetForumTopicIconStickers {} } }
