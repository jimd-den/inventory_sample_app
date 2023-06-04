use inv_tdd::core::entities::asset::Asset;
use inv_tdd::core::repositories::asset_repository::AssetRepository;
use inv_tdd::core::usecases::asset::add_asset::AddAssetUseCase;
use inv_tdd::core::entities::transaction::Transaction;
use inv_tdd::core::repositories::transaction_repository::TransactionRepository;
use inv_tdd::core::usecases::transaction::add_transaction::AddTransactionUseCase;
use inv_tdd::infrastructure::repositories::in_memory::in_memory_asset_repository::InMemoryAssetRepository;
use inv_tdd::infrastructure::repositories::in_memory::in_memory_transaction_repository::InMemoryTransactionRepository;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
mod core;

fn run_menu_ui() {
    println!("Inventory App");
    println!("1. Add Item");
    println!("2. add_transaction");
    println!("3. View Transactions");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: String = input.trim().to_string();
    match input.as_str() {
        "1" => {
            add_item_ui();
            run_menu_ui()
        }
        "2" => {
            add_transaction_ui();
            run_menu_ui()
        }
        "3" => {
            view_transactions_ui();
            run_menu_ui()
        }
        _ => println!("Command not found"),
    }
}

fn add_item_ui() {
    let mut repo = InMemoryAssetRepository::new();
    let mut add_asset_use_case = AddAssetUseCase::new(&mut repo);

    let mut name = String::new();
    let mut sku = String::new();
    let uuid = Uuid::new_v4();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| panic!("SystemTime before UNIX EPOCH!"))
        .as_secs()
        .to_string();

    println!("Enter Name: ");
    std::io::stdin().read_line(&mut name).expect("Failed to read input");
    println!("Enter SKU: ");
    std::io::stdin().read_line(&mut sku).expect("Failed to read input");
    println!("UUID: {}", uuid);
    println!("Timestamp: {}\n", timestamp);
    let asset = Asset::new(
        uuid,
        name.trim().to_string(),
        sku.trim().to_string(),
        timestamp.parse().expect("Failed to parse timestamp"),
    );
    add_asset_use_case.execute(asset);
    println!("##Item Added##\n");
}

fn add_transaction_ui() {
    let mut repo = InMemoryTransactionRepository::new();
    let mut add_transaction_use_case = AddTransactionUseCase::new(&mut repo);

    let mut asset_id = String::new();
    

}

fn view_transactions_ui() {}

fn cli_main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    match command.as_str() {
        "--menu" => {
            run_menu_ui();
        }
        _ => println!("Command not found"),
    }
}
fn main() {
    cli_main();
}
