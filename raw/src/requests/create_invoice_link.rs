use crate::requests::*;
use crate::types::*;

/// Use this method to create a link for an invoice.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct CreateInvoiceLink {
    title: String,
    description: String,
    payload: String,
    currency: String,
    prices: Vec<LabeledPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tip_amount: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_tip_amounts: Option<Vec<Integer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_size: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_phone_number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_shipping_address: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_phone_number_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_email_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_flexible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_period: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
}

impl Request for CreateInvoiceLink {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<String>;
    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("createInvoiceLink"), self)
    }
}

impl CreateInvoiceLink {
    pub fn new<T, D, P, C>(title: T, description: D, payload: P, currency: C, prices: Vec<LabeledPrice>) -> Self
    where T: Into<String>, D: Into<String>, P: Into<String>, C: Into<String>,
    {
        CreateInvoiceLink {
            title: title.into(), description: description.into(), payload: payload.into(),
            currency: currency.into(), prices, provider_token: None, max_tip_amount: None,
            suggested_tip_amounts: None, provider_data: None, photo_url: None, photo_size: None,
            photo_width: None, photo_height: None, need_name: None, need_phone_number: None,
            need_email: None, need_shipping_address: None, send_phone_number_to_provider: None,
            send_email_to_provider: None, is_flexible: None, subscription_period: None,
            business_connection_id: None,
        }
    }
}

/// This object represents a portion of the price for goods or services.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: Integer,
}

/// Use this method to refund a successful payment in Telegram Stars.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct RefundStarPayment {
    user_id: UserId,
    telegram_payment_charge_id: String,
}
impl Request for RefundStarPayment {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("refundStarPayment"), self) }
}
impl RefundStarPayment {
    pub fn new<U: ToUserId, S: Into<String>>(user: U, charge_id: S) -> Self {
        RefundStarPayment { user_id: user.to_user_id(), telegram_payment_charge_id: charge_id.into() }
    }
}

/// Answers a shipping query.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct AnswerShippingQuery {
    shipping_query_id: String,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    shipping_options: Option<Vec<ShippingOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<String>,
}
impl Request for AnswerShippingQuery {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("answerShippingQuery"), self) }
}
impl AnswerShippingQuery {
    pub fn ok<S: Into<String>>(id: S, options: Vec<ShippingOption>) -> Self {
        AnswerShippingQuery { shipping_query_id: id.into(), ok: true, shipping_options: Some(options), error_message: None }
    }
    pub fn error<S: Into<String>, E: Into<String>>(id: S, error: E) -> Self {
        AnswerShippingQuery { shipping_query_id: id.into(), ok: false, shipping_options: None, error_message: Some(error.into()) }
    }
}

/// This object represents one shipping option.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShippingOption {
    pub id: String,
    pub title: String,
    pub prices: Vec<LabeledPrice>,
}

/// Answers a pre-checkout query.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct AnswerPreCheckoutQuery {
    pre_checkout_query_id: String,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<String>,
}
impl Request for AnswerPreCheckoutQuery {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;
    fn serialize(&self) -> Result<HttpRequest, Error> { Self::Type::serialize(RequestUrl::method("answerPreCheckoutQuery"), self) }
}
impl AnswerPreCheckoutQuery {
    pub fn ok<S: Into<String>>(id: S) -> Self {
        AnswerPreCheckoutQuery { pre_checkout_query_id: id.into(), ok: true, error_message: None }
    }
    pub fn error<S: Into<String>, E: Into<String>>(id: S, error: E) -> Self {
        AnswerPreCheckoutQuery { pre_checkout_query_id: id.into(), ok: false, error_message: Some(error.into()) }
    }
}

/// Use this method to send invoices.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SendInvoice {
    chat_id: ChatRef,
    title: String,
    description: String,
    payload: String,
    currency: String,
    prices: Vec<LabeledPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tip_amount: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_tip_amounts: Option<Vec<Integer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_parameter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_size: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_phone_number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_shipping_address: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_phone_number_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_email_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_flexible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl Request for SendInvoice {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<MessageOrChannelPost>;
    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("sendInvoice"), self)
    }
}

impl SendInvoice {
    pub fn new<Ch, T, D, P, C>(chat: Ch, title: T, description: D, payload: P, currency: C, prices: Vec<LabeledPrice>) -> Self
    where Ch: ToChatRef, T: Into<String>, D: Into<String>, P: Into<String>, C: Into<String>,
    {
        SendInvoice {
            chat_id: chat.to_chat_ref(), title: title.into(), description: description.into(),
            payload: payload.into(), currency: currency.into(), prices,
            provider_token: None, max_tip_amount: None, suggested_tip_amounts: None,
            start_parameter: None, provider_data: None, photo_url: None, photo_size: None,
            photo_width: None, photo_height: None, need_name: None, need_phone_number: None,
            need_email: None, need_shipping_address: None, send_phone_number_to_provider: None,
            send_email_to_provider: None, is_flexible: None, disable_notification: None,
            protect_content: None, message_thread_id: None, reply_parameters: None, reply_markup: None,
        }
    }
}
