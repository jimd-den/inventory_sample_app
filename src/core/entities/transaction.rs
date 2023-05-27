use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Transaction {
    id: u32,
    date_time: String,
    quantity: u32,
    asset_id: Uuid,
    asset_sku: String,
}

impl Transaction {
    fn new(id: u32, date_time: String, quantity: u32, asset_id: Uuid, asset_sku: String) -> Self {
        Self {
            id,
            date_time,
            quantity,
            asset_id,
            asset_sku,
        }
    }   
}
