use crate::requests::*;
use crate::types::*;

/// Informs a user that some of the Telegram Passport elements they provided contains errors.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SetPassportDataErrors {
    user_id: UserId,
    errors: Vec<PassportElementError>,
}
impl Request for SetPassportDataErrors {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("setPassportDataErrors"), self) }
}
impl SetPassportDataErrors {
    pub fn new<U: ToUserId>(user: U, errors: Vec<PassportElementError>) -> Self {
        SetPassportDataErrors { user_id: user.to_user_id(), errors }
    }
}
