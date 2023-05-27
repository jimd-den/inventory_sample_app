use crate::core::entities::asset::Asset;
use crate::core::repositories::asset_repository::AssetRepository;
use uuid::Uuid;

pub struct AddAssetUseCase<'a > {
    repository: & 'a mut dyn AssetRepository,
}

impl< 'a> AddAssetUseCase<'a> {
    pub fn new(repository: & 'a mut dyn AssetRepository) -> Self {
        Self { repository }
    }

    pub fn execute(&mut self, asset: Asset) -> Result<(), Box<(dyn std::error::Error)>> {
        self.repository.create_asset(asset)
    }
}
