use std::time::Instant;

use ggez::{
    graphics::{Canvas, Color, DrawParam, Image, Rect},
    mint::{Point2, Vector2},
};

use crate::{
    errors::DodgerError,
    resources::Resources,
    utils::{validate_coordinates, RectSize},
};

pub struct FallingObject {
    pub coords: Point2<f32>,
    pub size: RectSize,
    pub scaling: Vector2<f32>,
    pub image: Image,
    pub is_good: bool,
    pub good_object_value: Option<GoodObjectValue>,
    pub remove_timer: Option<Instant>,
    pub blink_timer: Option<Instant>,
    pub alpha: f32,
    pub pulse_time: f32,
}

impl FallingObject {
    /// **Creates a new falling object with given properties.**
    ///
    /// ## Parameters
    /// * `coords`: initial position of the object on the screen.
    /// * `scaling`: scaling factor for the size of the object.
    /// * `is_good`: a boolean indicating whether the object is good (`true`) or bad (`false`).
    /// * `good_object_value`: the score value if the object is good.
    /// * `resources`: a reference to resources.
    ///
    /// ## Returns
    /// A result with the newly created `FallingObject`, or a `DodgerError` if coordinates validation fails.
    pub fn new(
        coords: Point2<f32>,
        scaling: Vector2<f32>,
        is_good: bool,
        good_object_value: Option<GoodObjectValue>,
        resources: &Resources,
    ) -> Result<Self, DodgerError> {
        let validated_coords = validate_coordinates(coords)?;

        let image = if is_good {
            match good_object_value {
                Some(GoodObjectValue::High) => &resources.good_object_high_image,
                Some(GoodObjectValue::Medium) => &resources.good_object_medium_image,
                Some(GoodObjectValue::Low) => &resources.good_object_low_image,
                None => &resources.good_object_low_image,
            }
        } else {
            &resources.bad_object_image
        };

        let w = image.width() as f32 * scaling.x;
        let h = image.height() as f32 * scaling.y;
        let size = RectSize::from((w, h));

        Ok(FallingObject {
            coords: validated_coords,
            size,
            scaling,
            image: image.clone(),
            good_object_value,
            is_good,
            remove_timer: None,
            blink_timer: None,
            alpha: 0.0,
            pulse_time: 0.0,
        })
    }

    /// **Updates position and handles behavior of a falling object.**
    ///
    /// ## Parameters
    /// * `resources`: a reference to resources.
    /// * `delta_time`: time since the last object update.
    ///
    /// ## Behavior
    /// * Updates position of the falling object based on the fall speed.
    /// * Handles special behavior for high-value good objects (pulsing effects).
    pub fn update(&mut self, resources: &Resources, delta_time: f32) {
        let speed = resources.level.fall_speed;
        self.coords.y += speed;

        if let Some(GoodObjectValue::High) = self.good_object_value {
            self.pulse_time += delta_time;
            if self.pulse_time > std::f32::consts::PI * 2.0 {
                self.pulse_time -= std::f32::consts::PI * 2.0;
            }
        }
    }

    /// **Draws a falling object.**
    ///
    /// ## Parameters
    /// `canvas`: canvas to draw the object on.
    ///
    /// ## Behavior
    /// Adjusts the transparency and scaling for good objects based on their type (blinking, pulsing).
    pub fn draw(&mut self, canvas: &mut Canvas) {
        let mut draw_params = DrawParam::default().dest(self.coords).scale(self.scaling);

        if let Some(timer) = self.blink_timer {
            let elapsed = timer.elapsed().as_secs_f32();
            let blink_speed = 10.0;

            self.alpha = (elapsed * blink_speed * std::f32::consts::PI).sin().abs();
            draw_params = draw_params.color(Color::new(1.0, 1.0, 1.0, self.alpha));
        }

        if self.is_good {
            if let Some(GoodObjectValue::High) = self.good_object_value {
                let pulse_factor = (self.pulse_time * 0.5).sin().abs() * 0.7 + 1.0;

                let high_scaling = Vector2 {
                    x: self.scaling.x * pulse_factor,
                    y: self.scaling.y * pulse_factor,
                };

                draw_params = draw_params
                    .scale(high_scaling)
                    .color(Color::new(1.0, 1.0, 0.5, 1.0));
            }
        }

        canvas.draw(&self.image, draw_params)
    }

    /// **Calculates the rectangular area occupied by the falling object.**
    ///
    /// ## Returns
    /// A `Rect` representing boundaries of the object based on its position and size on the screen.
    pub fn rect(&self) -> Rect {
        Rect::new(self.coords.x, self.coords.y, self.size.w, self.size.h)
    }
}

/// Value of a good falling object.
#[derive(PartialEq)]
pub enum GoodObjectValue {
    High,
    Medium,
    Low,
}

impl GoodObjectValue {
    /// **Returns the score associated with the type of good object.**
    ///
    /// ## Returns
    /// * `30`: for `High` value objects.
    /// * `15`: for `Medium` value objects.
    /// * `5`: for `Low` value objects.
    pub fn score(&self) -> i32 {
        match self {
            GoodObjectValue::High => 30,
            GoodObjectValue::Medium => 15,
            GoodObjectValue::Low => 5,
        }
    }
}
