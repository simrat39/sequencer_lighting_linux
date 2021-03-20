use omen_rgb_test::{
    animation::Animation,
    wave::{self, Color, CustomWaveColors, Wave},
};

fn main() {
    let h = hidapi::HidApi::new().unwrap();

    for device in h.device_list().skip(2) {
        let dev = device.open_device(&h).unwrap();

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

        for b in wave_buf {
            dev.write(b.as_slice()).unwrap();
        }

        println!("{:?}", dev.check_error().unwrap());
    }
}
