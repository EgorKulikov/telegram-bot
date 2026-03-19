use crate::types::*;

/// Parameters for a suggested post.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuggestedPostParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_price: Option<SuggestedPostPrice>,
}

/// Price for a suggested post.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuggestedPostPrice {
    pub star_count: Integer,
}

/// Info about a suggested post.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SuggestedPostInfo {
    pub suggest_date: Integer,
    pub price: Option<SuggestedPostPrice>,
}

/// Service message: a suggested post was approved.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SuggestedPostApproved {}

/// Service message: a suggested post approval failed.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SuggestedPostApprovalFailed {}

/// Service message: a suggested post was declined.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SuggestedPostDeclined {}

/// Service message: a suggested post was paid.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SuggestedPostPaid {}

/// Service message: a suggested post was refunded.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SuggestedPostRefunded {}
