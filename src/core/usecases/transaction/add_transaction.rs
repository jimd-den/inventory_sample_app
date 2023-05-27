use crate::core::entities::transaction::Transaction;
use crate::core::repositories::transaction_repository::TransactionRepository;
use uuid::Uuid;

pub struct AddTransaction {

    repository: & 'a mut TransactionRepository,
}

impl< 'a> AddTransaction<'a > {
    pub fn new(repository: & 'a mut dyn TransactionRepository) -> Self {
        Self { repository }
    }

    pub fn execute(&mut self, Transaction) -> Result<(), Box<(dyn std::error::Error)>> {
        self.repository.create_transaction(transaction)
    }


