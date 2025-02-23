use ggez::{
    event::MouseButton,
    graphics::{Color, Rect},
    mint::{Point2, Vector2},
    Context,
};

use crate::{
    buttons::{IconButton, TextButton},
    consts::{
        BUTTON_SPACING, BUTTON_TEXT_SIZE, OBJECT_SCALING, PLAYER_SCALING, TEXT_BUTTON_HEIGHT,
        TEXT_BUTTON_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH,
    },
    errors::DodgerError,
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

pub fn validate_coordinates(coords: Point2<f32>) -> Result<Point2<f32>, DodgerError> {
    if coords.x < 0.0 || coords.x > WINDOW_WIDTH || coords.y < 0.0 || coords.y > WINDOW_HEIGHT {
        Err(DodgerError::InvalidCoordinates(
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
    let x = (WINDOW_WIDTH / 2.0) - (TEXT_BUTTON_WIDTH / 2.0);
    let y = start_y + (button_index as f32 * (TEXT_BUTTON_HEIGHT + BUTTON_SPACING));

    Point2::from_slice(&[x, y])
}

pub fn get_level_button(
    level_index: usize,
    start_y: f32,
    font: String,
) -> Result<TextButton, DodgerError> {
    let button_coords = start_point_of_button_in_set(level_index, start_y);

    TextButton::new(
        button_coords,
        Color::WHITE,
        text_button_rectsize(),
        format!("Level {}", level_index + 1),
        Color::BLACK,
        BUTTON_TEXT_SIZE,
        font,
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

pub fn icon_button_rect(button: &IconButton) -> Result<Rect, DodgerError> {
    let button_coords = validate_coordinates(button.coords)?;
    let (w, h) = icon_button_size(button);
    Ok(Rect::new(button_coords.x, button_coords.y, w, h))
}

pub fn text_button_rect(button: &TextButton) -> Result<Rect, DodgerError> {
    let button_coords = validate_coordinates(button.coords)?;
    Ok(Rect::new(
        button_coords.x,
        button_coords.y,
        TEXT_BUTTON_WIDTH,
        TEXT_BUTTON_HEIGHT,
    ))
}

pub fn is_button_clicked(ctx: &mut Context, button_rect: Rect) -> bool {
    if ctx.mouse.button_pressed(MouseButton::Left) {
        let mouse_position = ctx.mouse.position();
        button_rect.contains(mouse_position)
    } else {
        false
    }
}
