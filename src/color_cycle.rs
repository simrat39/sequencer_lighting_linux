use crate::{
    animation::Animation, animation_custom_colors::AnimationCustomColors,
    animation_speed::AnimationSpeed,
};

pub struct ColorCycle {
    pub theme: Option<ColorCycleTheme>,
    pub speed: Option<AnimationSpeed>,
    pub custom_colors: Option<AnimationCustomColors>,
}

impl Animation for ColorCycle {
    fn get_modified_buf(&self) -> Vec<Vec<u8>> {
        let mut empty_buf = ColorCycle::get_empty_buf();
        let bytes = &mut empty_buf[1];

        // idk 1
        bytes[0] = 0x03;
        // idk 2
        bytes[2] = 0x16;
        // idk 3
        bytes[6] = 0x04;
        // animation
        bytes[1] = 0x01;
        // theme
        bytes[5] = self.binary_for_theme();
        // speed
        bytes[22] = self.binary_for_speed();

        // custom colors
        if let Some(ColorCycleTheme::Custom) = self.theme {
            if let Some(val) = &self.custom_colors {
                val.set_colors_in_buffer(bytes);
            }
        } else {
            eprintln!("Custom colors provided but theme is not custom. Ignoring colors as they override the theme!");
        }

        empty_buf
    }
}

impl ColorCycle {
    fn binary_for_theme(&self) -> u8 {
        match &self.theme {
            Some(value) => match value {
                ColorCycleTheme::Galaxy => 0x05,
                ColorCycleTheme::Volcano => 0x02,
                ColorCycleTheme::Jungle => 0x03,
                ColorCycleTheme::Ocean => 0x04,
                ColorCycleTheme::Custom => 0x01,
            },
            None => 0x05,
        }
    }

    fn binary_for_speed(&self) -> u8 {
        match &self.speed {
            Some(val) => AnimationSpeed::binary_for_speed(val),
            None => AnimationSpeed::default(),
        }
    }
}

pub enum ColorCycleTheme {
    Galaxy,
    Volcano,
    Jungle,
    Ocean,
    Custom,
}
