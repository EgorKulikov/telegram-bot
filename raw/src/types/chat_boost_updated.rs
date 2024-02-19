use crate::Chat;

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct ChatBoostUpdated {
    pub chat: Chat,
}