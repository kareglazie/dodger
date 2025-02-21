use ggez::graphics::Color;

pub const LIVES: u8 = 5;
pub const LEVEL_DURATION_SECS: u64 = 40;
pub const FALLING_OBJECT_UPDATE_MILLIS: u64 = 800;

// Dimensions
pub const WINDOW_WIDTH: f32 = 1000.0;
pub const WINDOW_HEIGHT: f32 = 800.0;
pub const TEXT_BUTTON_WIDTH: f32 = 200.0;
pub const TEXT_BUTTON_HEIGHT: f32 = 50.0;
pub const BUTTON_TEXT_SIZE: f32 = 30.0;
pub const BUTTON_SPACING: f32 = 10.0;
pub const PLAYER_SCALING: f32 = 0.4;
pub const OBJECT_SCALING: f32 = 0.08;

// Colors
pub const GREY: Color = Color::new(150.0, 150.0, 150.0, 1.0);
pub const PURPLE: Color = Color::new(128.0, 0.0, 128.0, 1.0);
pub const GRASS: Color = Color::new(51.0, 153.0, 102.0, 1.0);
pub const DARK_GREEN: Color = Color::new(200.0, 200.0, 200.0, 1.0);
pub const WHITE: Color = Color::new(1.0, 1.0, 1.0, 1.0);
pub const BLACK: Color = Color::new(255.0, 255.0, 255.0, 1.0);
