use crate::requests::*;
use crate::types::*;

/// Use this method to delete messages on behalf of a business account.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct DeleteBusinessMessages {
    business_connection_id: String,
    message_ids: Vec<MessageId>,
}
impl Request for DeleteBusinessMessages {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("deleteBusinessMessages"), self) }
}
impl DeleteBusinessMessages {
    pub fn new<S: Into<String>>(business_connection_id: S, message_ids: Vec<MessageId>) -> Self {
        DeleteBusinessMessages {
            business_connection_id: business_connection_id.into(),
            message_ids,
        }
    }
}
