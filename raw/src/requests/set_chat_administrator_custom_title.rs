use crate::requests::*;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SetChatAdministratorCustomTitle {
    chat_id: ChatRef,
    user_id: UserId,
    custom_title: String,
}

impl Request for SetChatAdministratorCustomTitle {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("setChatAdministratorCustomTitle"), self)
    }
}

impl SetChatAdministratorCustomTitle {
    pub fn new<C: ToChatRef, U: ToUserId, S: Into<String>>(chat: C, user: U, title: S) -> Self {
        SetChatAdministratorCustomTitle {
            chat_id: chat.to_chat_ref(), user_id: user.to_user_id(), custom_title: title.into(),
        }
    }
}
