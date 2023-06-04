use crate::core::entities::asset::Asset;
use std::error::Error;

pub trait AssetRepository {
    fn create_asset(&mut self, asset: Asset) -> Result<(), Box<dyn Error>>;
    fn get_asset(&self, asset_id: &str) -> Result<Asset, Box<dyn Error>>;
}
