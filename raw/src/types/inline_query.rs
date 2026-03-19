use crate::types::*;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct InlineQuery {
    pub id: InlineQueryId,
    pub from: User,
    pub query: String,
    pub offset: String,
    pub chat_type: Option<String>,
    pub location: Option<Location>,
}

impl Into<InlineQueryId> for InlineQuery {
    fn into(self) -> InlineQueryId {
        self.id
    }
}
