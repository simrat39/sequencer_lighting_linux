use crate::{
    animation::animation_custom_colors::AnimationCustomColors,
    animation::animation_speeds::AnimationSpeed, animation::animation_themes::AnimationThemes,
    animation::Animation,
};

pub struct LineStreak {
    pub theme: Option<AnimationThemes>,
    pub speed: Option<AnimationSpeed>,
    pub custom_colors: Option<AnimationCustomColors>,
}

impl Animation for LineStreak {
    fn get_modified_buf(&self) -> Vec<Vec<u8>> {
        let mut empty_buf = LineStreak::get_empty_buf();
        let bytes = &mut empty_buf[1];

        // idk 1
        bytes[0] = 0x03;
        // idk 2
        bytes[2] = 0x16;
        // idk 3
        bytes[6] = 0x04;
        // animation
        bytes[1] = 0x07;
        // speed
        bytes[22] = self.binary_for_speed();

        // theme
        AnimationThemes::set_theme(&self.theme, bytes, false);

        // custom colors
        if let Some(AnimationThemes::Custom) = self.theme {
            if let Some(val) = &self.custom_colors {
                val.set_colors_in_buffer(bytes);
            }
        } else {
            eprintln!("Custom colors provided but theme is not custom. Ignoring colors as they override the theme!");
        }

        empty_buf
    }
}

impl LineStreak {
    fn binary_for_speed(&self) -> u8 {
        match &self.speed {
            Some(val) => AnimationSpeed::binary_for_speed(val),
            None => AnimationSpeed::default(),
        }
    }
}
