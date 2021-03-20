use crate::color::Color;

#[derive(Debug)]
pub struct AnimationCustomColors {
    pub colors: Vec<Color>,
}

impl AnimationCustomColors {
    pub fn set_colors_in_buffer(&self, buf: &mut Vec<u8>) {
        if self.colors.is_empty() {
            panic!("Colors are empty, this function shouldnt be called");
        }

        let len = &self.colors.len();
        if *len > 4 {
            eprintln!("More than 4 custom colors provided, only the first 4 will be considered");
        }

        buf[6] = (len - 1) as u8;

        for (index, color) in self.colors.iter().enumerate() {
            match index {
                0 => {
                    buf[10] = color.r;
                    buf[11] = color.g;
                    buf[12] = color.b;
                }
                1 => {
                    buf[13] = color.r;
                    buf[14] = color.g;
                    buf[15] = color.b;
                }
                2 => {
                    buf[16] = color.r;
                    buf[17] = color.g;
                    buf[18] = color.b;
                }
                3 => {
                    buf[19] = color.r;
                    buf[20] = color.g;
                    buf[21] = color.b;
                }
                _ => {}
            }
        }
    }
}
