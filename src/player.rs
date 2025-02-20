use std::time::Instant;

use ggez::{
    graphics::{draw, drawable_size, Color, DrawParam, Image, Rect},
    mint::{Point2, Vector2},
    Context, GameResult,
};

use crate::utils::RectSize;

#[derive(Clone)]
pub struct Player {
    pub coords: Point2<f32>,
    pub size: RectSize,
    pub scaling: Vector2<f32>,
    pub image: Image,
    pub blink_timer: Option<Instant>,
    pub alpha: f32,
}

impl Player {
    pub fn new(
        _ctx: &mut Context,
        coords: Point2<f32>,
        scaling: Vector2<f32>,
        image: &Image,
    ) -> Self {
        let w = image.width() as f32 * scaling.x;
        let h = image.width() as f32 * scaling.x;
        let size = RectSize::from((w, h));
        Player {
            coords,
            size,
            scaling,
            image: image.clone(),
            blink_timer: None,
            alpha: 0.0,
        }
    }

    pub fn move_left(&mut self) {
        self.coords.x -= 20.0_f32.max(0.0);
    }

    pub fn move_right(&mut self, ctx: &mut Context) {
        let (screen_width, _) = drawable_size(ctx);

        self.coords.x += 20.0_f32.min(screen_width - self.size.w);
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut draw_params = DrawParam::default().dest(self.coords).scale(self.scaling);

        if let Some(timer) = self.blink_timer {
            let elapsed = timer.elapsed().as_secs_f32();
            let blink_speed = 10.0; // Частота мигания при столкновении (сколько раз в секунду)

            self.alpha = (elapsed * blink_speed * std::f32::consts::PI).sin().abs();
            draw_params = draw_params.color(Color::new(1.0, 1.0, 1.0, self.alpha));
        }

        draw(ctx, &self.image, draw_params)
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.coords.x, self.coords.y, self.size.w, self.size.h)
    }
}
