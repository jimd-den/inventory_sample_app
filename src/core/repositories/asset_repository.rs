use std::error::Error;

pub trait AssetRepository {
    fn create_asset(&self, asset: Asset) -> Result<(), Box<dyn Error>>;
}
