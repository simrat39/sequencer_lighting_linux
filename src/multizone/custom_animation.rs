use super::{animation_api::AnimationApi, Multizone};

pub trait CustomAnimation {
    fn get_animation_api(&self) -> AnimationApi;

    fn start<F: Multizone>(&self) {
        let api = self.get_animation_api();
        api.run_animation_loop();
    }
}
