use crate::requests::*;
use crate::types::*;

/// Use this method to change the tag of a chat member.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SetChatMemberTag {
    chat_id: ChatRef,
    user_id: UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<String>,
}
impl Request for SetChatMemberTag {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("setChatMemberTag"), self) }
}
impl SetChatMemberTag {
    pub fn new<C: ToChatRef, U: ToUserId>(chat: C, user: U) -> Self {
        SetChatMemberTag { chat_id: chat.to_chat_ref(), user_id: user.to_user_id(), tag: None }
    }
    pub fn tag<S: Into<String>>(&mut self, tag: S) -> &mut Self { self.tag = Some(tag.into()); self }
}
