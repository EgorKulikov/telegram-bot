use crate::types::*;

/// This object contains information about a Telegram Star transaction.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct StarTransaction {
    pub id: String,
    pub amount: Integer,
    pub nanostar_amount: Option<Integer>,
    pub date: Integer,
    pub source: Option<TransactionPartner>,
    pub receiver: Option<TransactionPartner>,
}

/// This object describes the source of a transaction, or its recipient for outgoing transactions.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type")]
pub enum TransactionPartner {
    #[serde(rename = "user")]
    User {
        user: User,
        invoice_payload: Option<String>,
        paid_media: Option<Vec<super::paid_media::PaidMedia>>,
    },
    #[serde(rename = "chat")]
    Chat { chat: Chat },
    #[serde(rename = "fragment")]
    Fragment {
        withdrawal_state: Option<RevenueWithdrawalState>,
    },
    #[serde(rename = "telegram_api")]
    TelegramApi { request_count: Integer },
    #[serde(rename = "telegram_ads")]
    TelegramAds,
    #[serde(rename = "other")]
    Other,
}

/// This object describes the state of a revenue withdrawal operation.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type")]
pub enum RevenueWithdrawalState {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "succeeded")]
    Succeeded { date: Integer, url: String },
    #[serde(rename = "failed")]
    Failed,
}

/// Contains a list of Telegram Star transactions.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct StarTransactions {
    pub transactions: Vec<StarTransaction>,
}
