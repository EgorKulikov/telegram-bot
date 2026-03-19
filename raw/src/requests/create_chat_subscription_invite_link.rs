use crate::requests::*;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct CreateChatSubscriptionInviteLink {
    chat_id: ChatRef,
    name: Option<String>,
    subscription_period: Integer,
    subscription_price: Integer,
}
impl Request for CreateChatSubscriptionInviteLink {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<ChatInviteLink>;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("createChatSubscriptionInviteLink"), self) }
}
impl CreateChatSubscriptionInviteLink {
    pub fn new<C: ToChatRef>(chat: C, period: Integer, price: Integer) -> Self {
        CreateChatSubscriptionInviteLink { chat_id: chat.to_chat_ref(), name: None, subscription_period: period, subscription_price: price }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct EditChatSubscriptionInviteLink {
    chat_id: ChatRef,
    invite_link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}
impl Request for EditChatSubscriptionInviteLink {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<ChatInviteLink>;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("editChatSubscriptionInviteLink"), self) }
}
impl EditChatSubscriptionInviteLink {
    pub fn new<C: ToChatRef, S: Into<String>>(chat: C, invite_link: S) -> Self {
        EditChatSubscriptionInviteLink { chat_id: chat.to_chat_ref(), invite_link: invite_link.into(), name: None }
    }
}
