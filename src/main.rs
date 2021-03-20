use omen_rgb_test::{
    animation::Animation,
    wave::{self, Color, CustomWaveColors, Wave},
};

fn main() {
    let hid = hidapi::HidApi::new().unwrap();

    for device in hid.device_list() {
        if device.vendor_id() == 1008 && device.product_id() == 8001 {
            let dev = device.open_device(&hid).unwrap();

            let config = Wave {
                speed: Some(wave::WaveSpeed::Medium),
                direction: Some(wave::WaveDirection::Down),
                theme: Some(wave::WaveTheme::Galaxy),
                custom_colors: Some(CustomWaveColors {
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
