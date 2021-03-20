#[derive(Debug)]
pub enum AnimationSpeed {
    Slow,
    Medium,
    Fast,
}

impl AnimationSpeed {
    pub fn binary_for_speed(speed: &AnimationSpeed) -> u8 {
        match &speed {
            AnimationSpeed::Slow => 0x00,
            AnimationSpeed::Medium => 0x01,
            AnimationSpeed::Fast => 0x02,
        }
    }

    pub fn default() -> u8 {
        0x01
    }
}
