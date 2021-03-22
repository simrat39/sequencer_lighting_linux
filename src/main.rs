use std::{thread, time::Duration};

use omen_rgb_test::{
    animation::{
        animation_custom_colors::AnimationCustomColors,
        animation_speeds::AnimationSpeed,
        animation_themes::AnimationThemes,
        wave::{Wave, WaveDirection},
        Animation,
    },
    color::Color,
    multizone::{
        full_static::{self, FullStatic},
        Multizone,
    },
};

fn main() {
    let hid = hidapi::HidApi::new().unwrap();

    for device in hid.device_list().skip(4) {
        if device.vendor_id() == 1008 && device.product_id() == 8001 {
            let dev = &device.open_device(&hid).unwrap();

            // let config = Wave {
            //     speed: Some(AnimationSpeed::Slow),
            //     theme: Some(AnimationThemes::Custom),
            //     custom_colors: Some(AnimationCustomColors {
            //         colors: vec![
            //             Color::from(0x00, 0xff, 0x00),
            //             Color::from(0xff, 0xff, 0xff),
            //             Color::from(0x00, 0x00, 0xff),
            //             Color::from(0xff, 0x00, 0x00),
            //         ],
            //     }),
            //     direction: Some(WaveDirection::Down),
            // };

            let config = FullStatic {
                color: Color::from(0xe5, 0x00, 0x00),
            };

            config.apply_effect(dev);
        }
    }
}
