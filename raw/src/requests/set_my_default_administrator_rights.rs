use crate::requests::*;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SetMyDefaultAdministratorRights {
    #[serde(skip_serializing_if = "Option::is_none")]
    rights: Option<ChatAdministratorRights>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_channels: Option<bool>,
}
impl Request for SetMyDefaultAdministratorRights {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("setMyDefaultAdministratorRights"), self) }
}
impl SetMyDefaultAdministratorRights {
    pub fn new() -> Self { SetMyDefaultAdministratorRights { rights: None, for_channels: None } }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetMyDefaultAdministratorRights {
    #[serde(skip_serializing_if = "Option::is_none")]
    for_channels: Option<bool>,
}
impl Request for GetMyDefaultAdministratorRights {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<ChatAdministratorRights>;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("getMyDefaultAdministratorRights"), self) }
}
impl GetMyDefaultAdministratorRights {
    pub fn new() -> Self { GetMyDefaultAdministratorRights { for_channels: None } }
}
