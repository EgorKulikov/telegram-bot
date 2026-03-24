use crate::types::*;

/// This object represent a user's profile audios.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct UserProfileAudios {
    pub total_count: Integer,
    pub audios: Vec<Audio>,
}
