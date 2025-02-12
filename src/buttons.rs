use ggez::graphics::{Color, Image};

#[derive(Clone)]
pub struct IconButton {
    pub coords_dest: Coordinates,
    pub coords_scale: Coordinates,
    pub icon: Image,
}
impl IconButton {
    pub fn new(coords_dest: Coordinates, coords_scale: Coordinates, icon: Image) -> Self {
        Self {
            coords_dest,
            coords_scale,
            icon,
        }
    }
}

#[derive(Clone)]
pub struct TextButton {
    pub coords_dest: Coordinates,
    pub size: RectSize,
    pub text: String,
    pub text_size: f32,
    pub text_color: Color,
    pub button_color: Color,
}

impl TextButton {
    pub fn new(
        coords_dest: Coordinates,
        size: RectSize,
        text: String,
        text_size: f32,
        text_color: Color,
        button_color: Color,
    ) -> Self {
        Self {
            coords_dest,
            size,
            text,
            text_size,
            text_color,
            button_color,
        }
    }
}

pub struct DrawText {
    pub coords_dest: Coordinates,
    pub text: String,
    pub size: f32,
    pub color: Color,
}

impl DrawText {
    pub fn new(coords_dest: Coordinates, text: String, size: f32, color: Color) -> Self {
        Self {
            coords_dest,
            text,
            size,
            color,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Coordinates {
    pub x: f32,
    pub y: f32
}

impl From<(f32, f32)> for Coordinates {
    fn from(coords: (f32, f32)) -> Self {
        Self { x: coords.0, y: coords.1 }
    }
}

#[derive(Clone, Copy)]
pub struct RectSize {
    pub width: f32,
    pub height: f32
}

impl From<(f32, f32)> for RectSize {
    fn from(size: (f32, f32)) -> Self {
        Self { width: size.0, height: size.1 }
    }
}