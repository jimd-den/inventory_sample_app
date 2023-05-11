use uuid::Uuid;

pub struct Product {
    product_uuid: Uuid,
    name: String,
    product_sku: String,
    quantity: i32,
    min_required: i32,
}

impl Product {
    pub fn new(product_id: Uuid, name: String, product_sku: String, quantity: i32, min_required: i32) -> Self {
        Product {
            product_id,
            name,
            product_sku,
            quantity,
            min_required,
        }
    }
}


