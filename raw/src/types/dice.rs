use crate::types::*;

/// This object represents an animated emoji that displays a random value.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Dice {
    pub emoji: String,
    pub value: Integer,
}
