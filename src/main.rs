use omen_rgb_test::{
    color::Color,
    multizone::{
        custom_animation::CustomAnimation, full_static::FullStatic, mid_shimmer::MidShimmer,
        per_key::PerKey,
    },
};

fn main() {
    let hid = hidapi::HidApi::new().unwrap();
    let mut omen_devices = Vec::new();

    for device in hid.device_list() {
        if device.vendor_id() == 1008 && device.product_id() == 8001 {
            omen_devices.push(device);
        }
    }

    let last_omen_device = omen_devices.iter().last().unwrap();
    let dev = last_omen_device.open_device(&hid).unwrap();

    let a = MidShimmer {
        fg_color: Color::from(255, 255, 0),
        bg_color: Color::from(0, 255, 255),
        device: &dev,
    };

    a.start::<FullStatic>();
}
