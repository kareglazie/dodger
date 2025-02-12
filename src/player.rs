use ggez::{
    graphics::{self, Image},
    Context, GameResult,
};

#[derive(Clone)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub image: Image,
}

impl Player {
    pub fn new(_ctx: &mut Context, x: f32, y: f32, image: &Image) -> Self {
        Player {
            x,
            y,
            width: image.width() as f32 * 0.2,
            height: image.height() as f32 * 0.2,
            image: image.clone(),
        }
    }

    pub fn move_left(&mut self) {
        self.x -= 15.0_f32.max(0.0);
    }

    pub fn move_right(&mut self, ctx: &mut Context) {
        let (screen_width, _) = graphics::drawable_size(ctx);

        self.x += 15.0_f32.min(screen_width - self.width);
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let draw_params = graphics::DrawParam::default()
            .dest(ggez::mint::Point2 {
                x: self.x,
                y: self.y,
            })
            .scale(ggez::mint::Vector2 { x: 0.2, y: 0.2 });
        graphics::draw(ctx, &self.image, draw_params)
    }

    pub fn rect(&self) -> graphics::Rect {
        graphics::Rect::new(self.x, self.y, self.width, self.height)
    }
}
