use crate::requests::*;
use crate::types::*;

/// Returns the bot's Telegram Star transactions in chronological order.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetStarTransactions {
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<Integer>,
}

impl Request for GetStarTransactions {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<StarTransactions>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("getStarTransactions"), self)
    }
}

impl GetStarTransactions {
    pub fn new() -> Self {
        GetStarTransactions {
            offset: None,
            limit: None,
        }
    }

    pub fn offset(&mut self, offset: Integer) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    pub fn limit(&mut self, limit: Integer) -> &mut Self {
        self.limit = Some(limit);
        self
    }
}
