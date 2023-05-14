use crate::core::entities::product::Product;
use crate::core::repositories::product_repository::ProductRepository;
use crate::core::use_cases::product_add::{AddProduct, AddProductUseCase};
use crate::infrastructure::repositories::in_memory_product_repository::InMemoryProductRepository;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "inventory_cmd", version = "1.0", author = "jimd")]
struct Args {

}

#[derive(Subcommand, Debug)]
enum SubCommands {
    add_product,
}

fn run_cli {
    let args = Args::parse();
    match args.subcmd {
        SubCommands::add_product => {
            let product_repository: Box<dyn ProductRepository> = Box::new(InMemoryProductRepository::new());
            let add_product_use_case: Box<dyn AddProduct> = Box::new(AddProductUseCase::new(product_repository));
            let product = Product::new("product1".to_string(), 1.0);
            add_product_use_case.execute(product);
        }
    
}
