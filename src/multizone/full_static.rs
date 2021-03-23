use crate::color::Color;

use super::Multizone;

pub struct FullStatic {
    pub color: Color,
}

impl Multizone for FullStatic {
    fn get_modified_buf(&self) -> Vec<Vec<u8>> {
        let mut empty = self.get_empty_buf();

        for i in 3..6 {
            for j in 4..64 {
                *empty.get_mut(i).unwrap().get_mut(j).unwrap() = self.color.r as u8;
            }
        }

        for i in 6..9 {
            for j in 4..64 {
                *empty.get_mut(i).unwrap().get_mut(j).unwrap() = self.color.g as u8;
            }
        }

        for i in 9..12 {
            for j in 4..64 {
                *empty.get_mut(i).unwrap().get_mut(j).unwrap() = self.color.b as u8;
            }
        }

        empty
    }
}
