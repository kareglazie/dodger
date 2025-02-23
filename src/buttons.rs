use crate::{
    errors::DodgerError,
    utils::{validate_coordinates, RectSize},
};
use ggez::{
    graphics::{Color, Image, PxScale, Text, TextFragment},
    mint::{Point2, Vector2},
};

#[derive(Clone)]
/// A button with an icon.
pub struct IconButton {
    pub coords: Point2<f32>,
    pub scaling: Vector2<f32>,
    pub icon: Image,
}

impl IconButton {
    /// **Creates a new `IconButton`.**
    ///
    /// ## Parameters
    /// * `coords`: coordinates of the button.
    /// * `scaling`: scaling vector for the icon.
    /// * `icon`: image icon to be displayed on the button.
    ///
    /// ## Returns
    /// A result containing the new `IconButton`, or a `DodgerError` if coordinates validation fails.
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
/// A button with text.
pub struct TextButton {
    pub coords: Point2<f32>,
    pub button_color: Color,
    pub button_size: RectSize,
    pub text: Text,
}

impl TextButton {
    /// **Creates a new `TextButton`.**
    ///
    /// ## Parameters
    /// * `coords`: coordinates of the button.
    /// * `button_color`: color of the button.
    /// * `button_size`: size of the button in terms of width and height.
    /// * `line`: text string to be displayed on the button.
    /// * `text_color`: color of the text.
    /// * `text_scale`: size of the text.
    /// * `font`: font for the text.
    ///
    /// ## Returns
    /// A result containing the new `TextButton`, or a `DodgerError` if coordinates validation fails.
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

/// A structure representing drawable text.
pub struct DrawText {
    pub coords: Point2<f32>,
    pub text: Text,
}

impl DrawText {
    /// **Creates a new `DrawText`.**
    ///
    /// ## Parameters
    /// * `coords`: coordinates where the text will be drawn.
    /// * `line`: text string to be drawn.
    /// * `font`: font for the text.
    /// * `scale`: size of the text.
    /// * `color`: color of the text.
    ///
    /// ## Returns
    /// A result containing the new `DrawText`, or a `DodgerError` if coordinates validation fails.
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
