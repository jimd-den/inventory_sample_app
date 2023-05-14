mod core;
mod infrastructure;
pub mod interfaces;

use crate::interfaces::cmd::run_cli;
use crate::core::entities::product::Product;
use crate::core::repositories::product_repository::ProductRepository;
use crate::core::use_cases::product_add::{AddProduct, AddProductUseCase};
use crate::infrastructure::repositories::in_memory_product_repository::InMemoryProductRepository;

fn main() {
    run_cli();

}