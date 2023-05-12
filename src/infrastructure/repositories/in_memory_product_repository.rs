use std::collections::HashMap; 
use std::sync::{Arc, RwLock};
use crate::core::entities::product::Product;
use crate::core::repositories::product_repository::ProductRepository;
use uuid::Uuid;


//inMemoryProductRepository is a data structure that holds a HashMap of products.
pub struct InMemoryProductRepository {
    storage: Arc<RwLock<HashMap<Uuid, Product>>>,
}

//inMemoryProductRepository implements the ProductRepository trait.
impl InMemoryProductRepository {
    pub fn new() -> Self {
            Self {
                storage: Arc::new(RwLock::new(HashMap::new())),
            }
      }
}

//inMemoryProductRepository implements the ProductRepository trait.

impl ProductRepository for InMemoryProductRepository {
    //this method creates a product and stores it in the HashMap storage.
    fn create_product(&self, product: Product) -> Product {
        let mut storage = self.storage.write().unwrap();
        storage.insert(uuid, product);
        Ok(())
    }
}

