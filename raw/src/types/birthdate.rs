use crate::types::*;

/// Describes the birthdate of a user.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Birthdate {
    pub day: Integer,
    pub month: Integer,
    pub year: Option<Integer>,
}
