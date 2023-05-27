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

    pub fn execute(&mut self, id: u32, date_time: String, quantity: u32, asset_id: Uuid, asset_sku: String) -> Result<(), Box<(dyn std::error::Error)>> {
        let transaction = Transaction::new(id, date_time, quantity, asset_id, asset_sku);
        self.repository.create_transaction(transaction)
    }


