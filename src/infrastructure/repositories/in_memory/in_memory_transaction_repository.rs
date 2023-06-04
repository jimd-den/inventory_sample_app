use crate::core::entities::transaction::Transaction;
use crate::core::repositories::transaction_repository::TransactionRepository;
use std::error::Error;
use std::collections::HashMap;
use uuid::Uuid;

pub struct InMemoryTransactionRepository {
    transactions: HashMap<Uuid, Transaction>,
}

impl InMemoryTransactionRepository {
    pub fn new() -> Self {
        InMemoryTransactionRepository {
            transactions: HashMap::new(),
        }
    }
    pub fn get_all_transactions(&self) -> &HashMap<Uuid, Transaction> {
        &self.transactions
    }
}

impl TransactionRepository for InMemoryTransactionRepository {
    fn create_transaction(&mut self, transaction: Transaction) -> Result<(), Box<dyn Error>> {
        self.transactions.insert(transaction.id, transaction.clone());
        Ok(())
    }
}