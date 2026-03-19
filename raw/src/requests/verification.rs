use crate::requests::*;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct VerifyUser {
    user_id: UserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_description: Option<String>,
}
impl Request for VerifyUser {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("verifyUser"), self) }
}
impl VerifyUser {
    pub fn new<U: ToUserId>(user: U) -> Self { VerifyUser { user_id: user.to_user_id(), custom_description: None } }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct VerifyChat {
    chat_id: ChatRef,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_description: Option<String>,
}
impl Request for VerifyChat {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("verifyChat"), self) }
}
impl VerifyChat {
    pub fn new<C: ToChatRef>(chat: C) -> Self { VerifyChat { chat_id: chat.to_chat_ref(), custom_description: None } }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct RemoveUserVerification { user_id: UserId }
impl Request for RemoveUserVerification {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("removeUserVerification"), self) }
}
impl RemoveUserVerification {
    pub fn new<U: ToUserId>(user: U) -> Self { RemoveUserVerification { user_id: user.to_user_id() } }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct RemoveChatVerification { chat_id: ChatRef }
impl Request for RemoveChatVerification {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("removeChatVerification"), self) }
}
impl RemoveChatVerification {
    pub fn new<C: ToChatRef>(chat: C) -> Self { RemoveChatVerification { chat_id: chat.to_chat_ref() } }
}
