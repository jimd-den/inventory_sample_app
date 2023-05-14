use uuid::Uuid;

use crate::core::use_cases::product_add;

#[derive(Debug, Clone)]
pub struct Product {
    product_uuid: Uuid,
    name: String,
    product_sku: String,
}

impl Product {
    pub fn new(name: String, product_sku: String) -> Self {
        let product_uuid = Uuid::new_v4();
        Product {
            product_uuid,
            name,
            product_sku,
        }
    }
}


