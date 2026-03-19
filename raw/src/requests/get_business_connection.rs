use crate::requests::*;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetBusinessConnection {
    business_connection_id: String,
}
impl Request for GetBusinessConnection {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<BusinessConnection>;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("getBusinessConnection"), self) }
}
impl GetBusinessConnection {
    pub fn new<S: Into<String>>(id: S) -> Self { GetBusinessConnection { business_connection_id: id.into() } }
}
