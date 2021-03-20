use crate::{
    animation::Animation, animation_custom_colors::AnimationCustomColors,
    animation_speed::AnimationSpeed,
};

#[derive(Debug)]
pub struct Wave {
    pub direction: Option<WaveDirection>,
    pub speed: Option<AnimationSpeed>,
    pub theme: Option<WaveTheme>,
    pub custom_colors: Option<AnimationCustomColors>,
}

impl Animation for Wave {
    fn get_modified_buf(&self) -> Vec<Vec<u8>> {
        let mut empty_buf = Wave::get_empty_buf();
        let bytes = &mut empty_buf[1];

        // idk 1
        bytes[0] = 0x03;
        // idk 2
        bytes[2] = 0x16;
        // animation
        bytes[1] = 0x06;
        // theme
        bytes[5] = self.binary_for_theme();
        // speed
        bytes[22] = self.binary_for_speed();
        // direction
        bytes[24] = self.binary_for_direction();

        // custom colors
        if let Some(WaveTheme::Custom) = self.theme {
            if let Some(val) = &self.custom_colors {
                val.set_colors_in_buffer(bytes);
            }
        } else {
            eprintln!("Custom colors provided but theme is not custom. Ignoring colors as they override the theme!");
        }

        empty_buf
    }
}

impl Wave {
    pub fn binary_for_direction(&self) -> u8 {
        match &self.direction {
            Some(dir) => match dir {
                WaveDirection::Left => 0x02,
                WaveDirection::Right => 0x03,
                WaveDirection::Up => 0x04,
                WaveDirection::Down => 0x05,
                WaveDirection::Inwards => 0x01,
                WaveDirection::Outwards => 0x00,
            },
            None => 0x03, // Right
        }
    }

    pub fn binary_for_speed(&self) -> u8 {
        match &self.speed {
            Some(val) => AnimationSpeed::binary_for_speed(val),
            None => AnimationSpeed::default(),
        }
    }

    fn binary_for_theme(&self) -> u8 {
        match &self.theme {
            Some(th) => match th {
                WaveTheme::Custom => 0x01,
                WaveTheme::Volcano => 0x02,
                WaveTheme::Jungle => 0x03,
                WaveTheme::Ocean => 0x04,
                WaveTheme::Galaxy => 0x05,
            },
            None => 0x05, // Galaxy
        }
    }
}

#[derive(Debug)]
pub enum WaveDirection {
    Left,
    Right,
    Up,
    Down,
    Inwards,
    Outwards,
}

#[derive(Debug)]
pub enum WaveTheme {
    Custom,
    Volcano,
    Jungle,
    Ocean,
    Galaxy,
}
