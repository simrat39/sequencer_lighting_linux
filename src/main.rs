use omen_rgb_test::multizone::{
    animated_gradient::AnimatedGradient, custom_animation::CustomAnimation, full_static::FullStatic,
};
use omen_rgb_test::{color::Color, multizone::shimmer::Shimmer};

fn main() {
    let hid = hidapi::HidApi::new().unwrap();
    for device in hid.device_list().skip(2) {
        if device.vendor_id() == 1008 && device.product_id() == 8001 {
            let dev = &device.open_device(&hid).unwrap();
            let a = Shimmer {
                fg_color: Color::from(0xff, 0xdf, 0x00),
                device: dev,
                bg_color: Color::from(0xad, 0xd8, 0xe6),
            };
            a.start::<FullStatic>();
        }
    }
}
