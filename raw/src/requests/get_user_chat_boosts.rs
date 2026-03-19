use crate::requests::*;
use crate::types::*;
use crate::types::chat_boost_updated::ChatBoost;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetUserChatBoosts {
    chat_id: ChatRef,
    user_id: UserId,
}
impl Request for GetUserChatBoosts {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<UserChatBoosts>;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("getUserChatBoosts"), self) }
}
impl GetUserChatBoosts {
    pub fn new<C: ToChatRef, U: ToUserId>(chat: C, user: U) -> Self {
        GetUserChatBoosts { chat_id: chat.to_chat_ref(), user_id: user.to_user_id() }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct UserChatBoosts {
    pub boosts: Vec<ChatBoost>,
}
