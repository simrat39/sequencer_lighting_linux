#[derive(Debug)]
pub enum AnimationThemes {
    Custom,
    Volcano,
    Jungle,
    Ocean,
    Galaxy,
}

impl AnimationThemes {
    pub fn set_theme(theme: &Option<AnimationThemes>, buf: &mut Vec<u8>, has_galaxy: bool) {
        if let Some(theme) = theme {
            theme.set_theme_in_buffer(buf, has_galaxy);
        } else {
            AnimationThemes::default().set_theme_in_buffer(buf, has_galaxy);
        }
    }

    fn set_theme_in_buffer(&self, buf: &mut Vec<u8>, has_galaxy: bool) {
        buf[5] = self.binary_for_theme(has_galaxy);
    }

    fn binary_for_theme(&self, has_galaxy: bool) -> u8 {
        match &self {
            AnimationThemes::Custom => 0x01,
            AnimationThemes::Volcano => 0x02,
            AnimationThemes::Jungle => 0x03,
            AnimationThemes::Ocean => 0x04,
            AnimationThemes::Galaxy => {
                if has_galaxy {
                    0x05
                } else {
                    eprintln!("Galaxy theme not supported. Using default (Ocean)");
                    AnimationThemes::default().binary_for_theme(has_galaxy)
                }
            }
        }
    }

    pub fn default() -> AnimationThemes {
        AnimationThemes::Ocean
    }
}
