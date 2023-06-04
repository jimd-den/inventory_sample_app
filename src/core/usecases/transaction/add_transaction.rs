use crate::core::entities::transaction::Transaction;
use crate::core::repositories::transaction_repository::TransactionRepository;
use uuid::Uuid;

pub struct AddTransactionUseCase<'a > {

    repository: &'a mut dyn TransactionRepository,
}

impl< 'a> AddTransactionUseCase<'a > {
    pub fn new(repository: & 'a mut dyn TransactionRepository) -> Self {
        Self { repository }
    }

    pub fn execute(&mut self,transaction: Transaction) -> Result<(), Box<(dyn std::error::Error)>> {
        self.repository.create_transaction(transaction)
    }

}


