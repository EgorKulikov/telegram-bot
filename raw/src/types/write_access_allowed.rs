/// This object represents a service message about a user allowing a bot to write messages.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct WriteAccessAllowed {
    pub from_request: Option<bool>,
    pub web_app_name: Option<String>,
    pub from_attachment_menu: Option<bool>,
}
