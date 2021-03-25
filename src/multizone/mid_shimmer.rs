use super::{
    animation_api::AnimationApi, custom_animation::CustomAnimation, key::Key, per_key::PerKey,
    Multizone,
};
use std::{collections::HashMap, time::Duration};
use strum::IntoEnumIterator;

use hidapi::HidDevice;

use crate::color::Color;

pub struct MidShimmer<'a> {
    pub fg_color: Color,
    pub bg_color: Color,
    pub device: &'a HidDevice,
}

impl<'a> CustomAnimation for MidShimmer<'a> {
    fn get_animation_api(&self) -> AnimationApi {
        let rows = Key::get_rows();
        const MID: usize = 11;

        let mut frames: Vec<Box<dyn Multizone>> = Vec::new();
        for i in 0..MID + 1 {
            let mut map: HashMap<Key, Color> = HashMap::new();

            for key in Key::iter() {
                map.insert(key, self.bg_color);
            }

            let left_rows = rows.iter().skip(MID - i).take(i);
            for row in left_rows {
                for key in row {
                    map.insert(*key, self.fg_color);
                }
            }

            let right_rows = rows.iter().skip(MID).take(i + 1);
            for row in right_rows {
                for key in row {
                    map.insert(*key, self.fg_color);
                }
            }

            frames.push(Box::new(PerKey { keys_colors: map }));
        }

        for i in 0..MID + 1 {
            let mut map: HashMap<Key, Color> = HashMap::new();

            for key in Key::iter() {
                map.insert(key, self.fg_color);
            }

            let left_rows = rows.iter().skip(MID - i).take(i);
            for row in left_rows {
                for key in row {
                    map.insert(*key, self.bg_color);
                }
            }

            let right_rows = rows.iter().skip(MID).take(i + 1);
            for row in right_rows {
                for key in row {
                    map.insert(*key, self.bg_color);
                }
            }

            frames.push(Box::new(PerKey { keys_colors: map }));
        }

        AnimationApi {
            frames,
            delay: Duration::from_millis(75),
            device: self.device,
        }
    }
}
