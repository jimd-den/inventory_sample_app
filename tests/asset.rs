use inv_tdd::core::entities::asset::Asset;
use inv_tdd::core::repositories::asset_repository::AssetRepository;
use inv_tdd::core::usecases::asset::add_asset::AddAssetUseCase;
use inv_tdd::infrastructure::repositories::in_memory_asset_repository::InMemoryAssetRepository;
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn can_add_product() {
    let mut repo = InMemoryAssetRepository::new();
    let mut add_asset_use_case = AddAssetUseCase::new(&mut repo);
    let id = Uuid::new_v4();    
    let name =  String::from("Asset 1");
    let sku = String::from("SKU-1");
    let date = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string();

    let asset = Asset::new(id, name, sku, date);
    add_asset_use_case.execute(asset.id, asset.name, asset.sku, asset.date_created);
    println!("HashMap: {:?}", repo.get_all_assets());

}
