use crate::{
    errors::DrawError,
    utils::{validate_coordinates, RectSize},
};
use ggez::{
    graphics::{Color, Image},
    mint::{Point2, Vector2},
};

#[derive(Clone)]
/// Кнопка с иконкой
pub struct IconButton {
    pub coords: Point2<f32>,
    pub scaling: Vector2<f32>,
    pub icon: Image,
}
impl IconButton {
    pub fn new(coords: Point2<f32>, scaling: Vector2<f32>, icon: Image) -> Result<Self, DrawError> {
        let validated_coords = validate_coordinates(coords)?;
        Ok(Self {
            coords: validated_coords,
            scaling,
            icon,
        })
    }
}

#[derive(Clone)]
/// Кнопка с текстом
pub struct TextButton {
    pub coords: Point2<f32>,
    pub size: RectSize,
    pub text: String,
    pub text_size: f32,
    pub text_color: Color,
    pub button_color: Color,
}

impl TextButton {
    pub fn new(
        coords: Point2<f32>,
        size: RectSize,
        text: String,
        text_size: f32,
        text_color: Color,
        button_color: Color,
    ) -> Result<Self, DrawError> {
        let validated_coords = validate_coordinates(coords)?;
        Ok(Self {
            coords: validated_coords,
            size,
            text,
            text_size,
            text_color,
            button_color,
        })
    }
}

pub struct DrawText {
    pub coords: Point2<f32>,
    pub text: String,
    pub size: f32,
    pub color: Color,
}

impl DrawText {
    pub fn new(
        coords: Point2<f32>,
        text: String,
        size: f32,
        color: Color,
    ) -> Result<Self, DrawError> {
        let validated_coords = validate_coordinates(coords)?;
        Ok(Self {
            coords: validated_coords,
            text,
            size,
            color,
        })
    }
}
