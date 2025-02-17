use ggez::{
    graphics::{drawable_size, Image},
    mint::{Point2, Vector2},
    Context,
};

use crate::utils::{DrawableObject, RectSize};

#[derive(Clone)]
pub struct Player {
    pub coords: Point2<f32>,
    pub size: RectSize,
    pub scaling: Vector2<f32>,
    pub image: Image,
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
        }
    }

    pub fn move_left(&mut self) {
        self.coords.x -= 15.0_f32.max(0.0);
    }

    pub fn move_right(&mut self, ctx: &mut Context) {
        let (screen_width, _) = drawable_size(ctx);

        self.coords.x += 15.0_f32.min(screen_width - self.size.w);
    }
}

impl DrawableObject for Player {
    fn coords(&self) -> Point2<f32> {
        self.coords
    }

    fn image(&self) -> &Image {
        &self.image
    }

    fn scaling(&self) -> Vector2<f32> {
        self.scaling
    }
    fn size(&self) -> &RectSize {
        &self.size
    }
}
