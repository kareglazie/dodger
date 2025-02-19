use std::time::Instant;

use ggez::{
    graphics::{draw, Color, DrawParam, Image, Rect},
    mint::{Point2, Vector2},
    Context, GameResult,
};

use crate::{resources::Resources, utils::RectSize};

pub struct FallingObject {
    pub coords: Point2<f32>,
    pub size: RectSize,
    pub scaling: Vector2<f32>,
    pub image: Image,
    pub is_good: bool,
    pub remove_timer: Option<Instant>,
    pub blink_timer: Option<Instant>,
    pub alpha: f32,
}

impl FallingObject {
    pub fn new(
        coords: Point2<f32>,
        scaling: Vector2<f32>,
        is_good: bool,
        resources: &Resources,
    ) -> Self {
        if is_good {
            let image = &resources.good_object_image;
            let w = image.width() as f32 * scaling.x;
            let h = image.height() as f32 * scaling.y;
            let size = RectSize::from((w, h));
            FallingObject {
                coords,
                size,
                scaling,
                image: image.clone(),
                is_good,
                remove_timer: None,
                blink_timer: None,
                alpha: 0.0,
            }
        } else {
            let image = &resources.bad_object_image;
            let w = image.width() as f32 * scaling.x;
            let h = image.height() as f32 * scaling.y;
            let size = RectSize::from((w, h));
            FallingObject {
                coords,
                size,
                scaling,
                image: image.clone(),
                is_good,
                remove_timer: None,
                blink_timer: None,
                alpha: 0.0,
            }
        }
    }

    pub fn update(&mut self, resources: &Resources) {
        let speed = resources.level.fall_speed;
        self.coords.y += 5.0 * speed * 0.5;
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut draw_params = DrawParam::default().dest(self.coords).scale(self.scaling);
        if let Some(timer) = self.blink_timer {
            let elapsed = timer.elapsed().as_secs_f32();
            let blink_speed = 10.0;

            self.alpha = (elapsed * blink_speed * std::f32::consts::PI).sin().abs();
            draw_params = draw_params.color(Color::new(1.0, 1.0, 1.0, self.alpha));
        }
        draw(ctx, &self.image, draw_params)
    }
    pub fn rect(&self) -> Rect {
        Rect::new(self.coords.x, self.coords.y, self.size.w, self.size.h)
    }
}
