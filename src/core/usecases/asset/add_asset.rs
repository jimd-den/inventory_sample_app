use core::entities::Asset;
use core::repositories::AssetRepository;
use uuid::Uuid;

pub struct AddAssetUseCase {
    repository: AssetRepository,
}

impl AddAssetUseCase {
    pub fn new(repository: AssetRepository) -> Self {
        Self { repository }
    }

    pub fn execute(&self, id: Uuid, name: String, sku:String. date_created: String) -> Result<Uuid, String> {
        let asset = Asset::new(id, name, sku, date_created);
        self.repository.add(asset)
    }
}
