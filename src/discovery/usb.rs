use crate::device;
use crate::device::Device;
use crate::udev::udev_device_attributes;

pub fn discover_devices_usb() -> Result<Vec<Device>, std::io::Error> {
    let mut devices: Vec<Device> = Vec::new();
    let usb_string: &str = "usb";
    let mut enumerator = udev::Enumerator::new()?;

    enumerator.match_subsystem(usb_string)?;
    let scanned_devices = enumerator.scan_devices()?;

    for device in scanned_devices {
        // devices.push(
        //     Device::new(
        //         device.attribute_value(udev_device_attributes::DeviceAttribute::VendorId.as_str()),
        //         2,
        //         Some(String::from("wdaw")),
        //         Some(String::from(device.sysname()))
        //     ));
    }
    Ok(devices)
}
