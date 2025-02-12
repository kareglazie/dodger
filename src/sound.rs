use ggez::audio::{SoundSource, Source};
use ggez::graphics::Image;
use ggez::Context;

pub struct AudioManager {
    pub speaker_icon: Image,
    pub speaker_muted_icon: Image,
    pub is_muted: bool,
}

impl AudioManager {
    pub fn new(ctx: &mut Context) -> Self {
        let speaker_icon = Image::new(ctx, "/Sounds/speaker.png").unwrap();
        let speaker_muted_icon = Image::new(ctx, "/Sounds/speaker_muted.png").unwrap();
        AudioManager {
            speaker_icon,
            speaker_muted_icon,
            is_muted: false,
        }
    }

    pub fn play_sound(&self, ctx: &mut Context, file_path: &str) {
        if self.is_muted {
            return;
        }
        let mut sound = Source::new(ctx, file_path).unwrap();
        sound.play_detached(ctx).unwrap();
    }

    pub fn toggle_mute(&mut self) {
        self.is_muted = !self.is_muted;
    }

    pub fn on_good_collision(&self, ctx: &mut Context) {
        self.play_sound(ctx, "/Sounds/success.ogg");
    }

    pub fn on_bad_collision(&self, ctx: &mut Context) {
        self.play_sound(ctx, "/Sounds/game-over.ogg");
    }
}
