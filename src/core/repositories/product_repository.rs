use crate::core::entities:product::Product;

//in clean architecture a repository is responsible for data access.
//with this ProductRepository, 

//we can create a product and store it in memory.
pub trait ProductRepository {
    async fn create_product(&self, product: Product) -> Result<()>;
}


