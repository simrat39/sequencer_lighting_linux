use omen_rgb_test::{
    animation::{
        animation_custom_colors::AnimationCustomColors,
        animation_speeds::AnimationSpeed,
        animation_themes::AnimationThemes,
        wave::{Wave, WaveDirection},
        Animation,
    },
    color::Color,
};

fn main() {
    let hid = hidapi::HidApi::new().unwrap();

    for device in hid.device_list() {
        if device.vendor_id() == 1008 && device.product_id() == 8001 {
            let dev = device.open_device(&hid).unwrap();

            let config = Wave {
                speed: Some(AnimationSpeed::Slow),
                theme: Some(AnimationThemes::Galaxy),
                custom_colors: Some(AnimationCustomColors {
                    colors: vec![
                        Color::from(0x00, 0xff, 0x00),
                        Color::from(0xff, 0xff, 0xff),
                        Color::from(0x00, 0x00, 0xff),
                        Color::from(0xff, 0x00, 0x00),
                    ],
                }),
                direction: Some(WaveDirection::Up),
            };

            // let config = Ripple {
            //     theme: Some(AnimationThemes::Custom),
            //     speed: Some(AnimationSpeed::Slow),
            //     custom_colors: Some(AnimationCustomColors {
            //         colors: vec![
            //             Color::from(0x00, 0xff, 0x00),
            //             Color::from(0xff, 0xff, 0xff),
            //             Color::from(0x00, 0x00, 0xff),
            //             Color::from(0xff, 0x00, 0x00),
            //         ],
            //     }),
            //     direction: Some(RippleSize::Big),
            // };

            config.apply_effect(dev);
        }
    }
}
