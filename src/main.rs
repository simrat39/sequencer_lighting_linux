use rand::{thread_rng, Rng};
use std::{collections::HashMap, thread, time::Duration};
use strum::IntoEnumIterator;

use omen_rgb_test::{
    color::Color,
    multizone::{key::Key, per_key::PerKey, Multizone},
};

fn main() {
    let hid = hidapi::HidApi::new().unwrap();

    for device in hid.device_list().skip(4) {
        if device.vendor_id() == 1008 && device.product_id() == 8001 {
            let dev = &device.open_device(&hid).unwrap();

            let mut rng = thread_rng();
            loop {
                let mut map = HashMap::new();
                for key in Key::iter() {
                    let r = rng.gen_range(0x00..0xff);
                    let g = 0xff - r;
                    let b = r / 2;
                    map.insert(key, Color::from(r as u8, g as u8, b as u8));
                }

                let config = PerKey { keys_colors: map };

                config.apply_effect(dev);
                thread::sleep(Duration::from_millis(200));
            }
        }
    }
}
