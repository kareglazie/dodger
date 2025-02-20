use std::time::Instant;

use ggez::{
    graphics::{draw, Color, DrawParam, Image, Rect},
    mint::{Point2, Vector2},
    Context,
};

use crate::{
    errors::DrawError,
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
    pub fn new(
        coords: Point2<f32>,
        scaling: Vector2<f32>,
        is_good: bool,
        good_object_value: Option<GoodObjectValue>,
        resources: &Resources,
    ) -> Result<Self, DrawError> {
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

    pub fn draw(&mut self, ctx: &mut Context) -> Result<(), DrawError> {
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

        draw(ctx, &self.image, draw_params).map_err(|err| DrawError::DrawObject(err.to_string()))
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.coords.x, self.coords.y, self.size.w, self.size.h)
    }
}

#[derive(PartialEq)]
pub enum GoodObjectValue {
    High,
    Medium,
    Low,
}

impl GoodObjectValue {
    pub fn score(&self) -> i32 {
        match self {
            GoodObjectValue::High => 30,
            GoodObjectValue::Medium => 15,
            GoodObjectValue::Low => 5,
        }
    }
}
