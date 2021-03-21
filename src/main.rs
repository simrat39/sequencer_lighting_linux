use omen_rgb_test::{
    animation::Animation, animation_custom_colors::AnimationCustomColors,
    animation_speeds::AnimationSpeed, animation_themes::AnimationThemes, color::Color,
    starlight::Starlight,
};

fn main() {
    let hid = hidapi::HidApi::new().unwrap();

    for device in hid.device_list() {
        if device.vendor_id() == 1008 && device.product_id() == 8001 {
            let dev = device.open_device(&hid).unwrap();

            // let config = wave::Wave {
            //     speed: Some(AnimationSpeed::Slow),
            //     direction: Some(wave::WaveDirection::Up),
            //     theme: Some(AnimationThemes::Galaxy),
            //     custom_colors: None,
            //     // custom_colors: Some(AnimationCustomColors {
            //     //     colors: vec![
            //     //         Color::from(0x00, 0xff, 0x00),
            //     //         Color::from(0xff, 0xff, 0xff),
            //     //         Color::from(0x00, 0x00, 0xff),
            //     //         Color::from(0xff, 0x00, 0x00),
            //     //     ],
            //     // }),
            // };

            let config = Starlight {
                theme: Some(AnimationThemes::Custom),
                speed: Some(AnimationSpeed::Fast),
                custom_colors: Some(AnimationCustomColors {
                    colors: vec![
                        Color::from(0x00, 0xff, 0x00),
                        Color::from(0xff, 0xff, 0xff),
                        Color::from(0x00, 0x00, 0xff),
                        Color::from(0xff, 0x00, 0x00),
                    ],
                }),
            };

            let wave_buf = config.get_modified_buf();

            for bytes in wave_buf {
                dev.write(bytes.as_slice()).unwrap();
            }
        }
    }
}
