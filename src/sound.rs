use ggez::{
    audio::{SoundSource, Source},
    graphics::Image,
    Context,
};
use std::collections::HashMap;

use crate::errors::DodgerError;

/// **Manages audio-related functionality and resources in the game.**
///
/// ## Features
/// * Handles the loading of sound effects.
/// * Manages speaker icons for mute/unmute functionality.
/// * Allows toggling of mute functionality and playing specific sounds.
pub struct AudioManager {
    pub speaker_icon: Image,
    pub speaker_muted_icon: Image,
    pub is_muted: bool,
    pub sounds: HashMap<String, String>,
}

impl AudioManager {
    /// **Creates a new `AudioManager` and initializes its resources.**
    ///
    /// ## Parameters
    /// `ctx`: the game context.
    ///
    /// ## Behavior
    /// * Loads the speaker and muted speaker icons.
    /// * Initializes a map containing predefined sound keys and file paths.
    ///
    /// ## Returns
    /// A result containing the `AudioManager`, or a `DodgerError` if any required resource (image or sound) fails to load.
    pub fn new(ctx: &mut Context) -> Result<Self, DodgerError> {
        let speaker_icon_path = "/Sounds/speaker.png";
        let speaker_icon_muted_path = "/Sounds/speaker_muted.png";

        let speaker_icon = Image::from_path(ctx, speaker_icon_path)
            .map_err(|_| DodgerError::InvalidImagePath(speaker_icon_path.to_string()))?;
        let speaker_muted_icon = Image::from_path(ctx, speaker_icon_muted_path)
            .map_err(|_| DodgerError::InvalidImagePath(speaker_icon_muted_path.to_string()))?;

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

    /// **Plays the sound effect corresponding to the specified key.**
    ///
    /// ## Parameters
    /// * `ctx`: the game context.
    /// * `sound_key`: the key corresponding to the desired sound in the audio manager's `sounds` map.
    ///
    /// ## Behavior
    /// * If the audio manager is muted, playback is skipped.
    /// * Retrieves the file path of the sound using the key and attempts to play it.
    ///
    /// ## Returns
    /// `Ok(())` if the sound is successfully played (or muted), or a `DodgerError` if the key is invalid or if there is an error playing the sound.
    pub fn play_sound(&self, ctx: &mut Context, sound_key: String) -> Result<(), DodgerError> {
        if self.is_muted {
            return Ok(());
        }

        if let Some(sound) = self.sounds.get(&sound_key) {
            let mut sound_source = Source::new(ctx, sound)
                .map_err(|_| DodgerError::InvalidSoundPath(sound.to_string()))?;
            sound_source
                .play_detached(ctx)
                .map_err(|err| DodgerError::AudioError(err.to_string()))?;

            Ok(())
        } else {
            Err(DodgerError::InvalidSoundKey(sound_key))
        }
    }

    /// **Toggles the mute state of the audio manager.**
    ///
    /// ## Behavior
    /// * If mute is active, no sound effects will play.
    /// * Changes `is_muted` to its opposite value.
    pub fn toggle_mute(&mut self) {
        self.is_muted = !self.is_muted;
    }
}
