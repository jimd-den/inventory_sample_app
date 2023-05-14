use crate::core::entities::product::Product;
use crate::core::repositories::product_repository::ProductRepository;
use crate::core::use_cases::product_add::{AddProduct, AddProductUseCase};
use crate::infrastructure::repositories::in_memory_product_repository::InMemoryProductRepository;
use clap::{Parser, Subcommand};
use std::io;

#[derive(Parser, Debug)]
#[clap(name = "inventory_cmd", version = "1.0", author = "jimd")]
struct Args {
    #[clap(subcommand)]
    subcmd: SubCommands,
}

#[derive(Parser, Debug)]
enum SubCommands {
    #[clap(name = "add_product")]
    add_product,
}

fn product_forum() {
    let mut loop_flag = true;
    while loop_flag {
        println!("What is the product_name?");
        let mut product_name = String::new();
        io::stdin().read_line(&mut product_name).expect("Failed to read line");
        if product_name.trim() == "exit" {
            loop_flag = false;
            break;
        }
        println!("What is the product_sku?");
        let mut product_sku = String::new();
        io::stdin().read_line(&mut product_sku).expect("Failed to read line");
        if product_sku.trim() == "exit" {
            loop_flag = false;
            break;
        }
        
        let storage = InMemoryProductRepository::new();
        let product = Product::new(product_name.trim().to_string(), product_sku.trim().to_string());
        let add_product = AddProductUseCase::new(storage);
        println!("Product to be added: {:?}", product);
        let product = add_product.execute(product);
        
    }
}

pub fn run_cli(){
    let args = Args::parse();
    match args.subcmd {
        SubCommands::add_product => product_forum(),
    }
}
