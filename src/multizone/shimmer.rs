use std::{collections::HashMap, time::Duration};

use hidapi::HidDevice;

use crate::color::Color;
use strum::IntoEnumIterator;

use super::{
    animation_api::AnimationApi, custom_animation::CustomAnimation, key::Key, per_key::PerKey,
    Multizone,
};

pub struct Shimmer<'a> {
    pub fg_color: Color,
    pub bg_color: Color,
    pub device: &'a HidDevice,
}

impl<'a> CustomAnimation for Shimmer<'a> {
    fn get_animation_api(&self) -> AnimationApi {
        let rows = Key::get_rows();

        let mut frames: Vec<Box<dyn Multizone>> = Vec::new();
        for i in 0..rows.len() {
            let mut map: HashMap<Key, Color> = HashMap::new();

            for key in Key::iter() {
                map.insert(key, self.bg_color);
            }

            for row in rows.iter().take(i) {
                for key in row {
                    map.insert(*key, self.fg_color);
                }
            }

            frames.push(Box::new(PerKey { keys_colors: map }));
        }

        for i in 0..rows.len() {
            let mut map: HashMap<Key, Color> = HashMap::new();

            for key in Key::iter() {
                map.insert(key, self.fg_color);
            }

            for row in rows.iter().take(i) {
                for key in row {
                    map.insert(*key, self.bg_color);
                }
            }

            frames.push(Box::new(PerKey { keys_colors: map }));
        }

        AnimationApi {
            frames,
            device: &self.device,
            delay: Duration::from_millis(30),
        }
    }
}
