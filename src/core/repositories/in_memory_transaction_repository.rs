use std::collections::HashMap;
use crate::core::entities::transaction::Transaction;
use crate::core::repositories::transaction_repository::TransactionRepository;
use uuid::Uuid;
use std::error::Error;

pub struct InMemoryTransactionRepository {
    transactions: HashMap<Uuid, Transaction>
}

impl inMemoryTransactionRepository {
    pub fn new() -> Self {
        in memoryTransactionRepository {
            transactions: HashMap::new()
        }
    }

    pub fn get_allTransactions(&self) -> &HashMap<Uuid, Transaction> {
        &self.transactions
    }

    impl TransactionRepository for inMemoryTransactionRepository {
        fn add(&mut self, transaction: Transaction) -> Result<(), Box<dyn Error>> {
            self.transactions.insert(transaction.id, transaction.clone());
            Ok(())
        }
    }
}
