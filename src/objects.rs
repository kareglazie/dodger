use ggez::{
    graphics::{self, Image},
    Context, GameResult,
};

use crate::resources::Resources;

pub struct FallingObject {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub image: Image,
    pub is_good: bool,
}

impl FallingObject {
    pub fn new(x: f32, y: f32, is_good: bool, resources: &Resources) -> Self {
        if is_good {
            let image = &resources.good_object_image;
            let width = image.width() as f32;
            let height = image.height() as f32;
            FallingObject {
                x,
                y,
                width: width * 0.08,
                height: height * 0.08,
                image: image.clone(),
                is_good,
            }
        } else {
            let image = &resources.bad_object_image;
            let width = image.width() as f32;
            let height = image.height() as f32;
            FallingObject {
                x,
                y,
                width: width * 0.08,
                height: height * 0.08,
                image: image.clone(),
                is_good,
            }
        }
    }

    pub fn update(&mut self, resources: &Resources) {
        let speed = resources.level.fall_speed;
        self.y += 5.0 * speed * 0.5;
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        // Отрисовываем изображение игрока
        let draw_params = graphics::DrawParam::default()
            .dest(ggez::mint::Point2 {
                x: self.x,
                y: self.y,
            })
            .scale(ggez::mint::Vector2 { x: 0.08, y: 0.08 });
        graphics::draw(ctx, &self.image, draw_params)
    }

    pub fn rect(&self) -> graphics::Rect {
        graphics::Rect::new(self.x, self.y, self.width, self.height)
    }
}
