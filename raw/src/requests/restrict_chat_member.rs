use crate::requests::*;
use crate::types::*;

/// Use this method to restrict a user in a supergroup.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct RestrictChatMember {
    chat_id: ChatRef,
    user_id: UserId,
    permissions: ChatPermissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_independent_chat_permissions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    until_date: Option<Integer>,
}

impl Request for RestrictChatMember {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("restrictChatMember"), self)
    }
}

impl RestrictChatMember {
    pub fn new<C: ToChatRef, U: ToUserId>(chat: C, user: U, permissions: ChatPermissions) -> Self {
        RestrictChatMember {
            chat_id: chat.to_chat_ref(), user_id: user.to_user_id(), permissions,
            use_independent_chat_permissions: None, until_date: None,
        }
    }
    pub fn until_date(&mut self, date: Integer) -> &mut Self { self.until_date = Some(date); self }
}
