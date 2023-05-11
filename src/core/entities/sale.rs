pub struct Sale {
    id: String,
    product_id: String,
    quantity: i32,
    sale_date: String,
}

impl Sale {
    pub fn new(id: String, product_id: String, quantity: i32, sale_date: String) -> Self {
        Sale {
            id,
            product_id,
            quantity,
            sale_date,
        }
    }
}
