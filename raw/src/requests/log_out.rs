use crate::requests::*;

/// Use this method to log out from the cloud Bot API server.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct LogOut {}

impl Request for LogOut {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("logOut"), self)
    }
}

impl LogOut { pub fn new() -> Self { LogOut {} } }

/// Use this method to close the bot instance before moving it from one local server to another.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct Close {}

impl Request for Close {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("close"), self)
    }
}

impl Close { pub fn new() -> Self { Close {} } }
