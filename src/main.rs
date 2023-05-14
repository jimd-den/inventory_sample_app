mod core;
mod infrastructure;
pub mod interfaces;

use crate::interfaces::cmd::run_cli;
use crate::core::entities::product::Product;
use crate::core::repositories::product_repository::ProductRepository;
use crate::core::use_cases::product_add::{AddProduct, AddProductUseCase};
use crate::infrastructure::repositories::in_memory_product_repository::InMemoryProductRepository;

fn main() {
    //create a new instance of the inMemoryProductRepository
    let storage = InMemoryProductRepository::new();

    //create a product
    let product = Product::new( "SKU-1".to_string(), "sku_one".to_string(), 5, 5);
    
    //use the add_product usecase to add the product to the repository
    let add_product = AddProductUseCase::new(storage);
    println!("Product to be added: {:?}", product);
    //
    let product = add_product.add_product(product);
    println!("Product added: {:?}", product);

}