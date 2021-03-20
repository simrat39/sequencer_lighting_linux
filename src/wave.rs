use crate::animation::Animation;

#[derive(Debug)]
pub struct Wave {
    pub direction: Option<WaveDirection>,
    pub speed: Option<WaveSpeed>,
    pub theme: Option<WaveTheme>,
    pub custom_colors: Option<CustomWaveColors>,
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
        if let Some(val) = &self.custom_colors {
            bytes[6] = (val.colors.len() - 1) as u8;

            for (index, color) in val.colors.iter().enumerate() {
                match index {
                    0 => {
                        bytes[10] = color.r;
                        bytes[11] = color.g;
                        bytes[12] = color.b;
                    }
                    1 => {
                        bytes[13] = color.r;
                        bytes[14] = color.g;
                        bytes[15] = color.b;
                    }
                    2 => {
                        bytes[16] = color.r;
                        bytes[17] = color.g;
                        bytes[18] = color.b;
                    }
                    3 => {
                        bytes[19] = color.r;
                        bytes[20] = color.g;
                        bytes[21] = color.b;
                    }
                    _ => {}
                }
            }
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

    fn binary_for_speed(&self) -> u8 {
        match &self.speed {
            Some(sp) => match sp {
                WaveSpeed::Slow => 0x00,
                WaveSpeed::Medium => 0x01,
                WaveSpeed::Fast => 0x02,
            },
            None => 0x01, // Medium
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
pub enum WaveSpeed {
    Slow,
    Medium,
    Fast,
}

#[derive(Debug)]
pub enum WaveTheme {
    Custom,
    Volcano,
    Jungle,
    Ocean,
    Galaxy,
}

#[derive(Debug)]
pub struct CustomWaveColors {
    pub colors: Vec<Color>,
}

#[derive(Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn from(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}

// pub fn get_wave(config: WaveConfig) -> Vec<Vec<u8>> {
//     // binary direction
//     let bd = config.binary_for_direction();
//     // binary speed
//     let bs = config.binary_for_speed();
//     // binary theme
//     let bt = config.binary_for_theme();

//     let r = 0xe5;
//     let g = 0x00;
//     let b = 0x50;

//     let wave: Vec<Vec<u8>> = vec![
//         vec![
//             0x09, 0x00, 0x01, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//         ],
//         vec![
//             0x03, 0x06, 0x16, 0x00, 0x00, bt, 0x00, r, g, b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, bs, 0x01, bd, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
//             0x00, 0x00, 0x00, 0x00, 0x00,
//         ],
//     ];
//     wave
// }
