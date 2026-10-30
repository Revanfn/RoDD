mod device;
mod discovery;
pub mod udev;

fn main() {
    let my_device = device::Device {
        vendor_id: 0x000a,
        product_id: 0x0001,
        product_name: Some(String::from("test")),
        manufacturer: Some(String::from("Logitech")),
    };

    println!("My Device: {}", my_device.product_name.as_deref().unwrap_or("unknown device"));

}
