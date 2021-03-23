use std::{collections::HashMap, time::Duration};

use hidapi::HidDevice;

use crate::color::Color;

use super::{
    animation_api::AnimationApi, custom_animation::CustomAnimation, key::Key, per_key::PerKey,
    Multizone,
};

pub struct AnimatedGradient<'a> {
    pub color1: Color,
    pub color2: Color,
    pub device: &'a HidDevice,
}

impl<'a> CustomAnimation for AnimatedGradient<'a> {
    fn get_animation_api(&self) -> AnimationApi {
        let mut frames: Vec<Box<dyn Multizone>> = Vec::new();

        let rows = Key::get_rows();

        // forward
        for i in 1..100 {
            let mut map: HashMap<Key, Color> = HashMap::new();

            // first four
            let first_four_color = self.color1.lerp(&self.color2, i);
            for row in rows.iter().take(4) {
                for key in row {
                    map.insert(*key, first_four_color);
                }
            }

            // next four
            let five_to_eight_color = self
                .color1
                .lerp(&self.color2, 20)
                .lerp(&self.color2.lerp(&self.color1, 20), i);
            for row in rows.iter().skip(4).take(4) {
                for key in row {
                    map.insert(*key, five_to_eight_color);
                }
            }

            // next four
            let nine_to_twelve_color = self
                .color1
                .lerp(&self.color2, 40)
                .lerp(&self.color2.lerp(&self.color1, 40), i);
            for row in rows.iter().skip(8).take(4) {
                for key in row {
                    map.insert(*key, nine_to_twelve_color);
                }
            }

            // next four
            let thirteen_to_sixteen_color = self
                .color1
                .lerp(&self.color2, 70)
                .lerp(&self.color2.lerp(&self.color1, 70), i);
            for row in rows.iter().skip(12).take(4) {
                for key in row {
                    map.insert(*key, thirteen_to_sixteen_color);
                }
            }

            // first four
            let last_colors = self.color2.lerp(&self.color1, i);
            for row in rows.iter().skip(16) {
                for key in row {
                    map.insert(*key, last_colors);
                }
            }

            frames.push(Box::new(PerKey { keys_colors: map }));
        }

        // reverse
        for i in 1..100 {
            let mut map: HashMap<Key, Color> = HashMap::new();

            // first four
            let first_four_color = self.color2.lerp(&self.color1, i);
            for row in rows.iter().take(4) {
                for key in row {
                    map.insert(*key, first_four_color);
                }
            }

            // next four
            let five_to_eight_color = self
                .color2
                .lerp(&self.color1, 20)
                .lerp(&self.color1.lerp(&self.color2, 20), i);
            for row in rows.iter().skip(4).take(4) {
                for key in row {
                    map.insert(*key, five_to_eight_color);
                }
            }

            // next four
            let nine_to_twelve_color = self
                .color2
                .lerp(&self.color1, 40)
                .lerp(&self.color1.lerp(&self.color2, 40), i);
            for row in rows.iter().skip(8).take(4) {
                for key in row {
                    map.insert(*key, nine_to_twelve_color);
                }
            }

            // next four
            let thirteen_to_sixteen_color = self
                .color2
                .lerp(&self.color1, 70)
                .lerp(&self.color1.lerp(&self.color2, 70), i);
            for row in rows.iter().skip(12).take(4) {
                for key in row {
                    map.insert(*key, thirteen_to_sixteen_color);
                }
            }

            // first four
            let last_colors = self.color1.lerp(&self.color2, i);
            for row in rows.iter().skip(16) {
                for key in row {
                    map.insert(*key, last_colors);
                }
            }

            frames.push(Box::new(PerKey { keys_colors: map }));
        }

        AnimationApi {
            frames,
            device: &self.device,
            delay: Duration::from_millis(35),
        }
    }
}
