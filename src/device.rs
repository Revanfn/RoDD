pub struct Device {
    pub vendor_id: u16,
    pub product_id: u16,
    pub manufacturer: Option<String>,
    pub product_name: Option<String>,
}

impl Device {
    pub fn new(
        vendor_id: u16,
        product_id: u16,
        manufacturer: Option<String>,
        product_name: Option<String>,
    ) -> Self {
        Self {
            vendor_id,
            product_id,
            manufacturer,
            product_name,
        }
    }
}
