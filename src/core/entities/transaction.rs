use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: Uuid,
    pub date_time: String,
    pub quantity: u32,
    pub asset_id: Uuid,
    pub asset_sku: String,
    pub location_from: String,
    pub location_to: String,
}

impl Transaction {
    fn new(id: Uuid, date_time: String, quantity: u32, asset_id: Uuid, asset_sku: String, location_from: String, location_to: String) -> Self {
        Self {
            id,
            date_time,
            quantity,
            asset_id,
            asset_sku,
            location_from,
            location_to,
        }
    }
}
