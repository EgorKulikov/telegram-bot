use crate::requests::*;
use crate::types::*;

/// Use this method to set default chat permissions for all members.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SetChatPermissions {
    chat_id: ChatRef,
    permissions: ChatPermissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_independent_chat_permissions: Option<bool>,
}

impl Request for SetChatPermissions {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("setChatPermissions"), self)
    }
}

impl SetChatPermissions {
    pub fn new<C>(chat: C, permissions: ChatPermissions) -> Self
    where
        C: ToChatRef,
    {
        SetChatPermissions {
            chat_id: chat.to_chat_ref(),
            permissions,
            use_independent_chat_permissions: None,
        }
    }
}
