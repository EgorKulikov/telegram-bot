use crate::requests::*;
use crate::types::*;

/// Use this method to ban a user in a group, a supergroup or a channel.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct BanChatMember {
    chat_id: ChatRef,
    user_id: UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    until_date: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revoke_messages: Option<bool>,
}

impl Request for BanChatMember {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("banChatMember"), self)
    }
}

impl BanChatMember {
    pub fn new<C, U>(chat: C, user: U) -> Self
    where
        C: ToChatRef,
        U: ToUserId,
    {
        BanChatMember {
            chat_id: chat.to_chat_ref(),
            user_id: user.to_user_id(),
            until_date: None,
            revoke_messages: None,
        }
    }

    pub fn until_date(&mut self, until_date: Integer) -> &mut Self {
        self.until_date = Some(until_date);
        self
    }

    pub fn revoke_messages(&mut self, revoke: bool) -> &mut Self {
        self.revoke_messages = Some(revoke);
        self
    }
}
