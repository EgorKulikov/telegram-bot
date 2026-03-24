use crate::requests::*;
use crate::types::*;

/// Use this method to get the bot's Telegram Star balance.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetMyStarBalance;
impl Request for GetMyStarBalance {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<StarAmount>;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("getMyStarBalance"), self) }
}
