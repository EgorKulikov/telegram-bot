use crate::requests::*;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SetMyName {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}
impl Request for SetMyName {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("setMyName"), self) }
}
impl SetMyName {
    pub fn new() -> Self { SetMyName { name: None, language_code: None } }
    pub fn name<S: Into<String>>(&mut self, name: S) -> &mut Self { self.name = Some(name.into()); self }
    pub fn language_code<S: Into<String>>(&mut self, code: S) -> &mut Self { self.language_code = Some(code.into()); self }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetMyName {
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}
impl Request for GetMyName {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<BotName>;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("getMyName"), self) }
}
impl GetMyName {
    pub fn new() -> Self { GetMyName { language_code: None } }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SetMyDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}
impl Request for SetMyDescription {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("setMyDescription"), self) }
}
impl SetMyDescription {
    pub fn new() -> Self { SetMyDescription { description: None, language_code: None } }
    pub fn description<S: Into<String>>(&mut self, desc: S) -> &mut Self { self.description = Some(desc.into()); self }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetMyDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}
impl Request for GetMyDescription {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<BotDescription>;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("getMyDescription"), self) }
}
impl GetMyDescription { pub fn new() -> Self { GetMyDescription { language_code: None } } }

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SetMyShortDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    short_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}
impl Request for SetMyShortDescription {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("setMyShortDescription"), self) }
}
impl SetMyShortDescription {
    pub fn new() -> Self { SetMyShortDescription { short_description: None, language_code: None } }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetMyShortDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}
impl Request for GetMyShortDescription {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<BotShortDescription>;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("getMyShortDescription"), self) }
}
impl GetMyShortDescription { pub fn new() -> Self { GetMyShortDescription { language_code: None } } }
