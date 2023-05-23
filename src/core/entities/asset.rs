use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Asset {
    pub id: Uuid,
    pub name: String,
    pub sku: String,
    pub date_created: String,
}

impl Asset {
    pub fn new(id: Uuid, name: String, sku: String, date_created: String) -> Asset {
        Asset {
            id,
            name,
            sku,
            date_created,
        }
    }
}
