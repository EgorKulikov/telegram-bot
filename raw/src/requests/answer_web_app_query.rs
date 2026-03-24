use crate::requests::*;
use crate::types::*;

/// Use this method to set the result of an interaction with a Web App and send
/// a corresponding message on behalf of the user to the chat from which the query originated.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct AnswerWebAppQuery {
    web_app_query_id: String,
    result: InlineQueryResult,
}
impl Request for AnswerWebAppQuery {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<SentWebAppMessage>;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("answerWebAppQuery"), self) }
}
impl AnswerWebAppQuery {
    pub fn new<S: Into<String>>(web_app_query_id: S, result: InlineQueryResult) -> Self {
        AnswerWebAppQuery { web_app_query_id: web_app_query_id.into(), result }
    }
}
