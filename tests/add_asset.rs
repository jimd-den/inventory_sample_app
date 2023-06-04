use inv_tdd::core::entities::asset::Asset;
use inv_tdd::core::repositories::asset_repository::AssetRepository;
use inv_tdd::core::usecases::asset::add_asset::AddAssetUseCase;
use inv_tdd::infrastructure::repositories::in_memory_asset_repository::InMemoryAssetRepository;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[test]
fn can_add_asset() {
    let mut repo = InMemoryAssetRepository::new();
    let mut add_asset_use_case = AddAssetUseCase::new(&mut repo);

    let asset = Asset::new(
        uuid::Uuid::new_v4(),
        "Asset 1".to_string(),
        "SKU-1".to_string(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string(),
    );

    add_asset_use_case.execute(asset);
    println!("HashMap: {:?}", repo.get_all_assets());
}
