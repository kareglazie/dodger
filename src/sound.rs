use ggez::{
    audio::{SoundSource, Source},
    graphics::Image,
    Context,
};
use std::collections::HashMap;

pub struct AudioManager {
    pub speaker_icon: Image,
    pub speaker_muted_icon: Image,
    pub is_muted: bool,
    pub sounds: HashMap<String, String>,
}

impl AudioManager {
    pub fn new(ctx: &mut Context) -> Self {
        let speaker_icon = Image::new(ctx, "/Sounds/speaker.png").unwrap();
        let speaker_muted_icon = Image::new(ctx, "/Sounds/speaker_muted.png").unwrap();
        let mut sounds = HashMap::new();
        sounds.insert(
            "good_collision".to_string(),
            "/Sounds/success.ogg".to_string(),
        );
        sounds.insert(
            "bad_collision".to_string(),
            "/Sounds/failure-alert.ogg".to_string(),
        );
        sounds.insert(
            "game_over".to_string(),
            "/Sounds/fail-trombone.ogg".to_string(),
        );
        sounds.insert(
            "level_completed".to_string(),
            "/Sounds/level-completed.ogg".to_string(),
        );
        sounds.insert("victory".to_string(), "/Sounds/fanfare.ogg".to_string());

        AudioManager {
            speaker_icon,
            speaker_muted_icon,
            is_muted: false,
            sounds,
        }
    }

    pub fn play_sound(&self, ctx: &mut Context, sound_key: String) {
        if self.is_muted {
            return;
        }
        let file_path = self.sounds.get(&sound_key).unwrap();
        let mut sound = Source::new(ctx, file_path).unwrap();
        sound.play_detached(ctx).unwrap();
    }

    pub fn toggle_mute(&mut self) {
        self.is_muted = !self.is_muted;
    }
}
