use std::{thread, time::Duration};

use hidapi::HidDevice;

use super::Multizone;

pub struct AnimationApi<'a> {
    pub frames: Vec<Box<dyn Multizone>>,
    pub device: &'a HidDevice,
    pub delay: Duration,
}

impl<'a> AnimationApi<'a> {
    pub fn run_animation_loop(&self) {
        loop {
            for frame in &self.frames {
                frame.apply_effect(&self.device);
                thread::sleep(self.delay);
            }
        }
    }
}
