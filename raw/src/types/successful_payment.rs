use crate::types::*;

/// This object contains basic information about a successful payment.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SuccessfulPayment {
    pub currency: String,
    pub total_amount: Integer,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: String,
    pub subscription_expiration_date: Option<Integer>,
    pub is_recurring: Option<bool>,
    pub is_first_recurring: Option<bool>,
}

/// This object contains basic information about a refunded payment.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct RefundedPayment {
    pub currency: String,
    pub total_amount: Integer,
    pub invoice_payload: String,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: Option<String>,
}

/// This object represents information about an order.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Invoice {
    pub title: String,
    pub description: String,
    pub start_parameter: Option<String>,
    pub currency: String,
    pub total_amount: Integer,
}

/// Service message about a price change for paid messages.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PaidMessagePriceChanged {
    pub paid_message_star_count: Integer,
}
