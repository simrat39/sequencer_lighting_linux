use omen_rgb_test::{
    buf::get_empty_bufs,
    wave::{self, Color, CustomWaveColors, WaveConfig},
};

fn main() {
    let h = hidapi::HidApi::new().unwrap();

    for device in h.device_list().skip(2) {
        let dev = device.open_device(&h).unwrap();

        let config = WaveConfig {
            speed: Some(wave::WaveSpeed::Slow),
            direction: Some(wave::WaveDirection::Up),
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

        let mut wave_buf = get_empty_bufs();
        config.from_bytes(&mut wave_buf[1]);

        for b in wave_buf {
            dev.write(b.as_slice()).unwrap();
        }

        println!("{:?}", dev.check_error().unwrap());
    }
}
