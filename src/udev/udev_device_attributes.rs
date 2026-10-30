pub enum DeviceAttribute {
    VendorId,
    ProductId,
    Manufacturer,
    ProductName,
    Serial,
}

impl DeviceAttribute {
    pub fn as_str(&self) -> &str {
        match self {
            DeviceAttribute::VendorId => "idVendor",
            DeviceAttribute::ProductId => "idProduct",
            DeviceAttribute::Manufacturer => "manufacturer",
            DeviceAttribute::ProductName => "product",
            DeviceAttribute::Serial => "serial",
        }
    }
}