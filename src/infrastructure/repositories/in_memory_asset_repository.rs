use crate::core::entities::asset::Asset;
use crate::core::repositories::asset_repository::AssetRepository;
use std::collections::HashMap;
use std::error::Error;
use uuid::Uuid;
pub struct InMemoryAssetRepository {
    assets: HashMap<Uuid, Asset>,
}

impl InMemoryAssetRepository {
    pub fn new() -> Self {
        InMemoryAssetRepository {
            assets: HashMap::new(),
        }
    }
    pub fn get_all_assets(&self) -> &HashMap<Uuid, Asset> {
        &self.assets
    }
}
impl AssetRepository for InMemoryAssetRepository {
    fn create_asset(&mut self, asset: Asset) -> Result<(), Box<dyn Error>> {
        self.assets.insert(asset.id, asset.clone());
        Ok(())
    }
}
