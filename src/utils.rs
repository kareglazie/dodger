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

/// **Size of a rectangle (width and height)**
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

/// **Validates if the coordinates are within the game window boundaries.**
///
/// ## Parameters
/// `coords`: a `Point2<f32>` representing the coordinates to validate.
///
/// ## Returns
/// A result containing validated coordinated, or a `DodgerError` if the coordinates are out of the valid range.
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

/// **Returns the size of a text button as a `RectSize` instance.**
pub fn text_button_rectsize() -> RectSize {
    RectSize::from((TEXT_BUTTON_WIDTH, TEXT_BUTTON_HEIGHT))
}

/// **Calculates the start coordinates of a centered button on the screen.**
///
/// ## Returns
/// Starting point of the centered button.
pub fn start_point_of_centered_button() -> Point2<f32> {
    Point2::from_slice(&[
        (WINDOW_WIDTH / 2.0) - (TEXT_BUTTON_WIDTH / 2.0),
        (WINDOW_HEIGHT / 2.0) - (TEXT_BUTTON_HEIGHT / 2.0),
    ])
}

/// **Calculates the start coordinates of a button in a set of buttons, spaced vertically.**
///
/// ## Parameters
/// * `button_index`: index of the button (0-based).
/// * `start_y`: the vertical starting point for the first button in the set.
///
/// ## Returns
/// Starting point of the button in the set.
pub fn start_point_of_button_in_set(button_index: usize, start_y: f32) -> Point2<f32> {
    let x = (WINDOW_WIDTH / 2.0) - (TEXT_BUTTON_WIDTH / 2.0);
    let y = start_y + (button_index as f32 * (TEXT_BUTTON_HEIGHT + BUTTON_SPACING));

    Point2::from_slice(&[x, y])
}

/// **Creates a level selection button.**
///
/// ## Parameters
/// * `level_index`: index of the level (0-based).
/// * `start_y`: vertical starting point for the first button in the set.
/// * `font`: font to be used for the button text.
///
/// ## Returns
/// A result containing a new `TextButton` instance, or a `DodgerError` if creation fails.
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

/// **Returns the default scaling vector for the player.**
pub fn player_scaling() -> Vector2<f32> {
    Vector2::from_slice(&[PLAYER_SCALING, PLAYER_SCALING])
}

/// **Returns the default scaling vector for objects.**
pub fn object_scaling() -> Vector2<f32> {
    Vector2::from_slice(&[OBJECT_SCALING, OBJECT_SCALING])
}

/// **Returns a scaling vector representing half scaling.**
pub fn half_scaling() -> Vector2<f32> {
    Vector2::from_slice(&[0.5, 0.5])
}

/// **Computes the size of an icon button based on its icon dimensions and scaling.**
///
/// ## Parameters
/// `button`: a reference to the `IconButton`.
///
/// ## Returns
/// The width and height of the icon button.
pub fn icon_button_size(button: &IconButton) -> (f32, f32) {
    (
        button.icon.width() as f32 * button.scaling.x,
        button.icon.height() as f32 * button.scaling.y,
    )
}

/// **Computes the rectangle representing the boundaries of an icon button.**
///
/// ## Parameters
/// `button`: a reference to the `IconButton`.
///
/// ## Returns
/// A result containing the rectangle representing the button's boundaries, or a `DodgerError` if the coordinates are invalid.
pub fn icon_button_rect(button: &IconButton) -> Result<Rect, DodgerError> {
    let button_coords = validate_coordinates(button.coords)?;
    let (w, h) = icon_button_size(button);
    Ok(Rect::new(button_coords.x, button_coords.y, w, h))
}

/// **Computes the rectangle representing the boundaries of a text button.**
///
/// ## Parameters
/// `button`: a reference to the `TextButton`.
///
/// ## Returns
/// A result containing the rectangle representing the button's boundaries, or a `DodgerError` if the coordinates are invalid.
pub fn text_button_rect(button: &TextButton) -> Result<Rect, DodgerError> {
    let button_coords = validate_coordinates(button.coords)?;
    Ok(Rect::new(
        button_coords.x,
        button_coords.y,
        TEXT_BUTTON_WIDTH,
        TEXT_BUTTON_HEIGHT,
    ))
}

/// **Checks if the button is clicked.**
///
/// ## Parameters
/// * `ctx`: the game context.
/// * `button_rect`: the rectangle representing the button's boundaries.
///
/// ## Returns
/// `True` if the button is clicked, `false` otherwise.
pub fn is_button_clicked(ctx: &mut Context, button_rect: Rect) -> bool {
    if ctx.mouse.button_pressed(MouseButton::Left) {
        let mouse_position = ctx.mouse.position();
        button_rect.contains(mouse_position)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_coordinates_valid() {
        let coords = Point2 { x: 100.0, y: 100.0 };
        assert!(validate_coordinates(coords).is_ok());
    }

    #[test]
    fn test_validate_coordinates_invalid_x() {
        let coords = Point2 { x: -10.0, y: 100.0 };
        assert!(validate_coordinates(coords).is_err());

        let coords = Point2 {
            x: WINDOW_WIDTH + 10.0,
            y: 100.0,
        };
        assert!(validate_coordinates(coords).is_err());
    }

    #[test]
    fn test_validate_coordinates_invalid_y() {
        let coords = Point2 { x: 100.0, y: -10.0 };
        assert!(validate_coordinates(coords).is_err());

        let coords = Point2 {
            x: 100.0,
            y: WINDOW_HEIGHT + 10.0,
        };
        assert!(validate_coordinates(coords).is_err());
    }

    #[test]
    fn test_start_point_of_centered_button() {
        let centered_point = start_point_of_centered_button();
        assert_eq!(
            centered_point,
            Point2 {
                x: (WINDOW_WIDTH / 2.0) - (TEXT_BUTTON_WIDTH / 2.0),
                y: (WINDOW_HEIGHT / 2.0) - (TEXT_BUTTON_HEIGHT / 2.0),
            }
        );
    }

    #[test]
    fn test_text_button_rect() {
        let text_button = TextButton::new(
            start_point_of_centered_button(),
            Color::WHITE,
            text_button_rectsize(),
            "Test Button".to_string(),
            Color::BLACK,
            48.0,
            "text_font".to_string(),
        )
        .unwrap();
        match text_button_rect(&text_button) {
            Ok(rect) => assert_eq!(
                rect,
                Rect::new(400.0, 375.0, TEXT_BUTTON_WIDTH, TEXT_BUTTON_HEIGHT)
            ),
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }
}
