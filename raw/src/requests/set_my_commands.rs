use crate::requests::*;
use crate::types::*;

/// Use this method to change the list of the bot's commands.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SetMyCommands {
    commands: Vec<BotCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

impl Request for SetMyCommands {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("setMyCommands"), self)
    }
}

impl SetMyCommands {
    pub fn new(commands: Vec<BotCommand>) -> Self {
        SetMyCommands {
            commands,
            scope: None,
            language_code: None,
        }
    }

    pub fn scope(&mut self, scope: BotCommandScope) -> &mut Self {
        self.scope = Some(scope);
        self
    }

    pub fn language_code<S: Into<String>>(&mut self, code: S) -> &mut Self {
        self.language_code = Some(code.into());
        self
    }
}

/// Use this method to get the current list of the bot's commands.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetMyCommands {
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

impl Request for GetMyCommands {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<Vec<BotCommand>>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("getMyCommands"), self)
    }
}

impl GetMyCommands {
    pub fn new() -> Self {
        GetMyCommands {
            scope: None,
            language_code: None,
        }
    }
}

/// Use this method to delete the list of the bot's commands.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct DeleteMyCommands {
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

impl Request for DeleteMyCommands {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("deleteMyCommands"), self)
    }
}

impl DeleteMyCommands {
    pub fn new() -> Self {
        DeleteMyCommands {
            scope: None,
            language_code: None,
        }
    }
}
