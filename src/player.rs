use std::time::Instant;

use ggez::{
    graphics::{Canvas, Color, DrawParam, Image, Rect},
    mint::{Point2, Vector2},
    Context,
};

use crate::{
    consts::WINDOW_WIDTH,
    errors::DodgerError,
    utils::{validate_coordinates, RectSize},
};

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
    ) -> Result<Self, DodgerError> {
        let validated_coords = validate_coordinates(coords)?;
        let w = image.width() as f32 * scaling.x;
        let h = image.width() as f32 * scaling.x;
        let size = RectSize::from((w, h));

        Ok(Player {
            coords: validated_coords,
            size,
            scaling,
            image: image.clone(),
            blink_timer: None,
            alpha: 0.0,
        })
    }

    pub fn move_left(&mut self) {
        self.coords.x -= 20.0_f32.max(0.0);
    }

    pub fn move_right(&mut self) {
        self.coords.x += 20.0_f32.min(WINDOW_WIDTH - self.size.w);
    }

    pub fn draw(&mut self, canvas: &mut Canvas) -> Result<(), DodgerError> {
        let mut draw_params = DrawParam::default().dest(self.coords).scale(self.scaling);

        if let Some(timer) = self.blink_timer {
            let elapsed = timer.elapsed().as_secs_f32();
            let blink_speed = 10.0; // Частота мигания при столкновении (сколько раз в секунду)

            self.alpha = (elapsed * blink_speed * std::f32::consts::PI).sin().abs();
            draw_params = draw_params.color(Color::new(1.0, 1.0, 1.0, self.alpha));
        }

        canvas.draw(&self.image, draw_params);
        Ok(())
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.coords.x, self.coords.y, self.size.w, self.size.h)
    }
}
