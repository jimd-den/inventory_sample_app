use crate::core::entities::transaction::Transaction;
use std::error::Error;

pub trait TransactionRepository {
    fn create_transaction(&mut self, transaction: Transaction) -> Result<(), Box<dyn Error>>;
}
