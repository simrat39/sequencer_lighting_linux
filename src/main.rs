use omen_rgb_test::color::Color;
use omen_rgb_test::multizone::{
    custom_animation::CustomAnimation, full_static::FullStatic, mid_shimmer::MidShimmer,
};

fn main() {
    let hid = hidapi::HidApi::new().unwrap();
    for device in hid.device_list().skip(2) {
        if device.vendor_id() == 1008 && device.product_id() == 8001 {
            let dev = &device.open_device(&hid).unwrap();
            let a = MidShimmer {
                fg_color: Color::from(0, 0xff, 0xff),
                device: dev,
                bg_color: Color::from(0xff, 0, 0),
            };
            a.start::<FullStatic>();
        }
    }
}
