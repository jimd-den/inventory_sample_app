use uuid::Uuid;

use crate::core::use_cases::product_add;

#[derive(Debug, Clone)]
pub struct Product {
    product_uuid: Uuid,
    name: String,
    product_sku: String,
    quantity: i32,
    min_required: i32,
}

impl Product {
    pub fn new(name: String, product_sku: String, quantity: i32, min_required: i32) -> Self {
        let product_uuid = Uuid::new_v4();
        Product {
            product_uuid,
            name,
            product_sku,
            quantity,
            min_required,
        }
    }
}


