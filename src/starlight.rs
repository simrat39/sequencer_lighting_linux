use crate::{
    animation::Animation, animation_custom_colors::AnimationCustomColors,
    animation_speed::AnimationSpeed,
};

pub struct Starlight {
    pub theme: Option<StarlightTheme>,
    pub speed: Option<AnimationSpeed>,
    pub custom_colors: Option<AnimationCustomColors>,
}

impl Animation for Starlight {
    fn get_modified_buf(&self) -> Vec<Vec<u8>> {
        let mut empty_buf = Starlight::get_empty_buf();
        let bytes = &mut empty_buf[1];

        // idk 1
        bytes[0] = 0x03;
        // idk 2
        bytes[2] = 0x16;
        // idk 3
        bytes[6] = 0x04;
        // animation
        bytes[1] = 0x02;
        // theme
        bytes[5] = self.binary_for_theme();
        // speed
        bytes[22] = self.binary_for_speed();

        // custom colors
        if let Some(StarlightTheme::Custom) = self.theme {
            if let Some(val) = &self.custom_colors {
                val.set_colors_in_buffer(bytes);
            }
        } else {
            eprintln!("Custom colors provided but theme is not custom. Ignoring colors as they override the theme!");
        }

        empty_buf
    }
}

impl Starlight {
    fn binary_for_theme(&self) -> u8 {
        match &self.theme {
            Some(value) => match value {
                StarlightTheme::Volcano => 0x02,
                StarlightTheme::Jungle => 0x03,
                StarlightTheme::Ocean => 0x04,
                StarlightTheme::Custom => 0x01,
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

pub enum StarlightTheme {
    Volcano,
    Jungle,
    Ocean,
    Custom,
}
