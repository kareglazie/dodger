use ggez::graphics::Color;

pub const LIVES: u8 = 5;
pub const LEVEL_DURATION_SECS: u64 = 40;
pub const FALLING_OBJECT_UPDATE_MILLIS: u64 = 800;
pub const YELLOW: Color = Color::new(153.0, 153.0, 0.0, 1.0);

// Dimensions
pub const WINDOW_WIDTH: f32 = 1000.0;
pub const WINDOW_HEIGHT: f32 = 800.0;

pub const TEXT_BUTTON_WIDTH: f32 = 200.0;
pub const TEXT_BUTTON_HEIGHT: f32 = 50.0;

pub const BUTTON_TEXT_SIZE: f32 = 26.0;
pub const TEXT_SIZE: f32 = 34.0;

pub const BUTTON_SPACING: f32 = 10.0;

pub const PLAYER_SCALING: f32 = 0.4;
pub const OBJECT_SCALING: f32 = 0.08;
