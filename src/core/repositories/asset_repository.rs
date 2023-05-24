use crate::core::entities::asset::Asset;
use std::error::Error;

pub trait AssetRepository {
    fn create_asset(&mut self, asset: Asset) -> Result<(), Box<dyn Error>>;
}
