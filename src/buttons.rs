use crate::{
    errors::DodgerError,
    utils::{validate_coordinates, RectSize},
};
use ggez::{
    graphics::{Color, Image, PxScale, Text, TextFragment},
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
    pub fn new(
        coords: Point2<f32>,
        scaling: Vector2<f32>,
        icon: Image,
    ) -> Result<Self, DodgerError> {
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
    pub button_color: Color,
    pub button_size: RectSize,
    pub text: Text,
}

impl TextButton {
    pub fn new(
        coords: Point2<f32>,
        button_color: Color,
        button_size: RectSize,
        line: String,
        text_color: Color,
        text_scale: f32,
        font: String,
    ) -> Result<Self, DodgerError> {
        let validated_coords = validate_coordinates(coords)?;
        let text = Text::new(TextFragment {
            text: line,
            font: Some(font),
            scale: Some(PxScale::from(text_scale)),
            color: Some(text_color),
        });
        Ok(Self {
            coords: validated_coords,
            button_color,
            button_size,
            text,
        })
    }
}

pub struct DrawText {
    pub coords: Point2<f32>,
    pub text: Text,
}

impl DrawText {
    pub fn new(
        coords: Point2<f32>,
        line: String,
        font: String,
        scale: f32,
        color: Color,
    ) -> Result<Self, DodgerError> {
        let validated_coords = validate_coordinates(coords)?;
        let text = Text::new(TextFragment {
            text: line,
            font: Some(font),
            scale: Some(PxScale::from(scale)),
            color: Some(color),
        });
        Ok(Self {
            coords: validated_coords,
            text,
        })
    }
}
