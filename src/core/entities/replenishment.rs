pub struct Replinish {
    replinish_id: String,
    product_id: String,
    quantity_replinshed: i32,
    replinish_date: String,
}

impl Replinish {
    pub fn new(replinish_id: String, product_id: String, quantity_replinshed: i32, replinish_date: String) -> Self {
        Replinish {
            replinish_id,
            product_id,
            quantity_replinshed,
            replinish_date,
        }
    }
}
