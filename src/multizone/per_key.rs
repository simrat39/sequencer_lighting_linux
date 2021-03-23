use std::collections::HashMap;

use crate::color::Color;

use super::{key::Key, Multizone};

pub struct PerKey {
    pub keys_colors: HashMap<Key, Color>,
}

impl Multizone for PerKey {
    fn get_modified_buf(&self) -> Vec<Vec<u8>> {
        let mut empty = PerKey::get_empty_buf();

        for (key, color) in self.keys_colors.iter() {
            let (i, j) = key.position_in_buffer_r();
            *empty
                .get_mut(i as usize)
                .unwrap()
                .get_mut(j as usize)
                .unwrap() = color.r;

            *empty
                .get_mut((i + 3) as usize)
                .unwrap()
                .get_mut(j as usize)
                .unwrap() = color.g;

            *empty
                .get_mut((i + 6) as usize)
                .unwrap()
                .get_mut(j as usize)
                .unwrap() = color.b;
        }
        empty
    }
}
