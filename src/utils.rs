use ggez::mint::Point2;

use crate::{
    consts::{WINDOW_HEIGHT, WINDOW_WIDTH},
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
