use crate::requests::*;
use crate::types::*;

/// Use this method to allow or disallow a user to proceed with a Star subscription.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct EditUserStarSubscription {
    user_id: UserId,
    telegram_payment_charge_id: String,
    is_canceled: bool,
}
impl Request for EditUserStarSubscription {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("editUserStarSubscription"), self) }
}
impl EditUserStarSubscription {
    pub fn new<U: ToUserId, S: Into<String>>(user: U, telegram_payment_charge_id: S, is_canceled: bool) -> Self {
        EditUserStarSubscription {
            user_id: user.to_user_id(),
            telegram_payment_charge_id: telegram_payment_charge_id.into(),
            is_canceled,
        }
    }
}
