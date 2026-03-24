use crate::types::*;

/// Describes an amount of Telegram Stars.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct StarAmount {
    pub amount: Integer,
    pub nanostar_amount: Option<Integer>,
}
