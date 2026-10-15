pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Order {
        if product_name.is_empty() {
            panic!("Product name cannot be empty");
        }

        if product_name.len() > 300 {
            panic!("Product name cannot be longer than 300 bytes");
        }

        if quantity == 0 {
            panic!("Quantity must be strictly greater than zero");
        }

        if unit_price == 0 {
            panic!("Unit price must be strictly greater than zero");
        }

        Order {
            product_name,
            quantity,
            unit_price,
        }
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }

    pub fn set_product_name(&mut self, product_name: String) {
        if product_name.is_empty() {
            panic!("Product name cannot be empty");
        }

        if product_name.len() > 300 {
            panic!("Product name cannot be longer than 300 bytes");
        }

        self.product_name = product_name;
    }

    pub fn set_quantity(&mut self, quantity: u32) {
        if quantity == 0 {
            panic!("Quantity must be strictly greater than zero");
        }

        self.quantity = quantity;
    }

    pub fn set_unit_price(&mut self, unit_price: u32) {
        if unit_price == 0 {
            panic!("Unit price must be strictly greater than zero");
        }

        self.unit_price = unit_price;
    }
}
