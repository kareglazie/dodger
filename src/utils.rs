use ggez::{
    event::MouseButton,
    graphics::{Color, Rect},
    input::mouse,
    mint::{Point2, Vector2},
    Context,
};

use crate::{
    buttons::{IconButton, TextButton},
    consts::{
        BUTTON_SPACING, BUTTON_TEXT_SIZE, OBJECT_SCALING, PLAYER_SCALING, TEXT_BUTTON_HEIGHT,
        TEXT_BUTTON_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH,
    },
    errors::DrawError,
};

/// Размер прямоугольника
#[derive(Clone, Copy)]
pub struct RectSize {
    pub w: f32,
    pub h: f32,
}

impl From<(f32, f32)> for RectSize {
    fn from(size: (f32, f32)) -> Self {
        Self {
            w: size.0,
            h: size.1,
        }
    }
}

pub fn validate_coordinates(coords: Point2<f32>) -> Result<Point2<f32>, DrawError> {
    if coords.x < 0.0 || coords.x > WINDOW_WIDTH || coords.y < 0.0 || coords.y > WINDOW_HEIGHT {
        Err(DrawError::InvalidCoordinates(
            coords.x,
            coords.y,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        ))
    } else {
        Ok(coords)
    }
}

pub fn text_button_rectsize() -> RectSize {
    RectSize::from((TEXT_BUTTON_WIDTH, TEXT_BUTTON_HEIGHT))
}

pub fn start_point_of_centered_button() -> Point2<f32> {
    Point2::from_slice(&[
        (WINDOW_WIDTH / 2.0) - (TEXT_BUTTON_WIDTH / 2.0),
        (WINDOW_HEIGHT / 2.0) - (TEXT_BUTTON_HEIGHT / 2.0),
    ])
}

pub fn start_point_of_button_in_set(button_index: usize, start_y: f32) -> Point2<f32> {
    Point2::from_slice(&[
        (WINDOW_WIDTH / 2.0) - (TEXT_BUTTON_WIDTH / 2.0),
        start_y + (button_index as f32 * (TEXT_BUTTON_HEIGHT + BUTTON_SPACING)),
    ])
}

pub fn get_level_button(level_index: usize, start_y: f32) -> Result<TextButton, DrawError> {
    TextButton::new(
        start_point_of_button_in_set(level_index, start_y),
        text_button_rectsize(),
        format!("Level {}", level_index + 1),
        BUTTON_TEXT_SIZE,
        Color::BLACK,
        Color::WHITE,
    )
}

pub fn player_scaling() -> Vector2<f32> {
    Vector2::from_slice(&[PLAYER_SCALING, PLAYER_SCALING])
}

pub fn object_scaling() -> Vector2<f32> {
    Vector2::from_slice(&[OBJECT_SCALING, OBJECT_SCALING])
}

pub fn half_scaling() -> Vector2<f32> {
    Vector2::from_slice(&[0.5, 0.5])
}

pub fn icon_button_size(button: &IconButton) -> (f32, f32) {
    (
        button.icon.width() as f32 * button.scaling.x,
        button.icon.height() as f32 * button.scaling.y,
    )
}

pub fn icon_button_rect(button: &IconButton) -> Rect {
    let (w, h) = icon_button_size(button);
    Rect::new(button.coords.x, button.coords.y, w, h)
}

pub fn text_button_rect(button: &TextButton) -> Rect {
    Rect::new(
        button.coords.x,
        button.coords.y,
        button.size.w,
        button.size.h,
    )
}

pub fn is_button_clicked(ctx: &mut Context, button_rect: Rect) -> bool {
    if mouse::button_pressed(ctx, MouseButton::Left) {
        let mouse_position = mouse::position(ctx);
        button_rect.contains(mouse_position)
    } else {
        false
    }
}
