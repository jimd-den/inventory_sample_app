use uuid::Uuid;
//date time import
use chrono::{DateTime, Utc};

pub enum TransactionType {
    Sale,
    Replinish,
}

pub struct Transaction {
    pub uuid: String,
    pub transaction_type: TransactionType,
    pub amount: f64,
    pub date: DateTime<Utc>,
}

impl Transaction {
    pub fn new(transaction_type: TransactionType, amount: f64) -> Transaction {
        Transaction {
            uuid: Uuid::new_v4().to_string(),
            transaction_type,
            amount,
            date: Utc::now(),
        }
    }
}
