use omen_rgb_test::{
    color::Color,
    multizone::{
        animated_gradient::AnimatedGradient, custom_animation::CustomAnimation,
        full_static::FullStatic,
    },
};

fn main() {
    let hid = hidapi::HidApi::new().unwrap();
    for device in hid.device_list().skip(2) {
        if device.vendor_id() == 1008 && device.product_id() == 8001 {
            let dev = &device.open_device(&hid).unwrap();
            let a = AnimatedGradient {
                color1: Color::from(0, 255, 0),
                color2: Color::from(0, 255, 255),
                device: dev,
            };
            a.start::<FullStatic>();
        }
    }
}
