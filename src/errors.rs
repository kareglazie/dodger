use ggez::GameError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DodgerError {
    #[error("No file path matching sound key in resources: {0}")]
    InvalidSoundKey(String),

    #[error("Failed to play sound {0}")]
    AudioError(String),

    #[error("Failed to load sound by path: {0}")]
    InvalidSoundPath(String),

    #[error("Failed to load image by path: {0}")]
    InvalidImagePath(String),

    #[error("Failed to load font by path: {0}")]
    InvalidFontPath(String),

    #[error("Unexpected error occurred")]
    Unexpected,

    #[error("The coordinates (x={0}, y={1}) are beyond the game field boundaries. Game field size is {2} - {3}")]
    InvalidCoordinates(f32, f32, f32, f32),

    #[error("Failed to render a player, error: {0}")]
    DrawPlayer(String),

    #[error("Failed to render an object, error: {0}")]
    DrawObject(String),

    #[error("Failed to render background, error: {0}")]
    DrawBackground(String),

    #[error("Failed to render text: {0}, error: {1}")]
    DrawText(String, String),

    #[error("Failed to render a countdown timer, error: {0}")]
    DrawTimer(String),

    #[error("Failed to render score: {0}, error: {1}")]
    DrawScore(String, String),

    #[error("Failed to render an icon button, error: {0}")]
    DrawIconButton(String),

    #[error("Failed to render a text button, error: {0}")]
    DrawTextButton(String),

    #[error("Failed to build a rectangle, error: {0}")]
    BuildRect(String),

    #[error("Failed to build an ellipse, error: {0}")]
    BuildEllipse(String),

    #[error("Failed to render a rectangle, error: {0}")]
    DrawRect(String),

    #[error("Failed to render an ellipse, error: {0}")]
    DrawEllipse(String),
}

impl From<DodgerError> for GameError {
    fn from(err: DodgerError) -> Self {
        GameError::CustomError(format!("{}", err))
    }
}
