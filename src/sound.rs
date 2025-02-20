use ggez::{
    audio::{SoundSource, Source},
    graphics::Image,
    Context,
};
use std::collections::HashMap;

use crate::errors::ResourceError;

pub struct AudioManager {
    pub speaker_icon: Image,
    pub speaker_muted_icon: Image,
    pub is_muted: bool,
    pub sounds: HashMap<String, String>,
}

impl AudioManager {
    pub fn new(ctx: &mut Context) -> Result<Self, ResourceError> {
        let speaker_icon_path = "/Sounds/speaker.png";
        let speaker_icon_muted_path = "/Sounds/speaker_muted.png";
        let speaker_icon = Image::new(ctx, speaker_icon_path)
            .map_err(|_| ResourceError::InvalidImagePath(speaker_icon_path.to_string()))?;
        let speaker_muted_icon = Image::new(ctx, speaker_icon_muted_path)
            .map_err(|_| ResourceError::InvalidImagePath(speaker_icon_muted_path.to_string()))?;
        let mut sounds = HashMap::new();
        sounds.insert(
            "good_collision".to_string(),
            "/Sounds/success.ogg".to_string(),
        );
        sounds.insert(
            "good_collision_high".to_string(),
            "/Sounds/treasure.ogg".to_string(),
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

        Ok(AudioManager {
            speaker_icon,
            speaker_muted_icon,
            is_muted: false,
            sounds,
        })
    }

    pub fn play_sound(&self, ctx: &mut Context, sound_key: String) -> Result<(), ResourceError> {
        if self.is_muted {
            return Ok(());
        }
        if let Some(sound) = self.sounds.get(&sound_key) {
            let mut sound_source = Source::new(ctx, sound)
                .map_err(|_| ResourceError::InvalidSoundPath(sound.to_string()))?;
            sound_source
                .play_detached(ctx)
                .map_err(|err| ResourceError::AudioError(err.to_string()))?;
            Ok(())
        } else {
            Err(ResourceError::InvalidSoundKey(sound_key))
        }
    }

    pub fn toggle_mute(&mut self) {
        self.is_muted = !self.is_muted;
    }
}
