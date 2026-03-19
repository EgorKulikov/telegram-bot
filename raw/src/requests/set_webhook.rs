use crate::requests::*;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SetWebhook {
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_connections: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_updates: Option<Vec<AllowedUpdate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drop_pending_updates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_token: Option<String>,
}

impl Request for SetWebhook {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("setWebhook"), self)
    }
}

impl SetWebhook {
    pub fn new<S: Into<String>>(url: S) -> Self {
        SetWebhook {
            url: url.into(), certificate: None, ip_address: None, max_connections: None,
            allowed_updates: None, drop_pending_updates: None, secret_token: None,
        }
    }
    pub fn secret_token<S: Into<String>>(&mut self, token: S) -> &mut Self {
        self.secret_token = Some(token.into()); self
    }
    pub fn ip_address<S: Into<String>>(&mut self, ip: S) -> &mut Self {
        self.ip_address = Some(ip.into()); self
    }
    pub fn max_connections(&mut self, max: Integer) -> &mut Self {
        self.max_connections = Some(max); self
    }
    pub fn allowed_updates(&mut self, updates: Vec<AllowedUpdate>) -> &mut Self {
        self.allowed_updates = Some(updates); self
    }
    pub fn drop_pending_updates(&mut self, drop: bool) -> &mut Self {
        self.drop_pending_updates = Some(drop); self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct DeleteWebhook {
    #[serde(skip_serializing_if = "Option::is_none")]
    drop_pending_updates: Option<bool>,
}

impl Request for DeleteWebhook {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("deleteWebhook"), self)
    }
}

impl DeleteWebhook {
    pub fn new() -> Self { DeleteWebhook { drop_pending_updates: None } }
    pub fn drop_pending_updates(&mut self, drop: bool) -> &mut Self {
        self.drop_pending_updates = Some(drop); self
    }
}

/// Contains information about the current status of a webhook.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct WebhookInfo {
    pub url: String,
    pub has_custom_certificate: bool,
    pub pending_update_count: Integer,
    pub ip_address: Option<String>,
    pub last_error_date: Option<Integer>,
    pub last_error_message: Option<String>,
    pub last_synchronization_error_date: Option<Integer>,
    pub max_connections: Option<Integer>,
    pub allowed_updates: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetWebhookInfo {}

impl Request for GetWebhookInfo {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<WebhookInfo>;
    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("getWebhookInfo"), self)
    }
}

impl GetWebhookInfo { pub fn new() -> Self { GetWebhookInfo {} } }
