use ggez::{
    graphics::{draw, DrawParam, Image, Rect},
    mint::{Point2, Vector2},
    Context, GameResult,
};

/// Размер прямоугольника
#[derive(Clone, Copy)]
pub struct RectSize {
    pub w: f32,
    pub h: f32,
}

impl From<(f32, f32)> for RectSize {
    fn from(size: (f32, f32)) -> Self {
        Self {
            w: size.0,
            h: size.1,
        }
    }
}

/// Трейт с дефолтной имплементацией рисования и расчета границ объекта
pub trait DrawableObject {
    fn coords(&self) -> Point2<f32>;
    fn image(&self) -> &Image;
    fn scaling(&self) -> Vector2<f32>;
    fn size(&self) -> &RectSize;

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let draw_params = DrawParam::default()
            .dest(self.coords())
            .scale(self.scaling());
        draw(ctx, self.image(), draw_params)
    }

    fn rect(&self) -> Rect {
        Rect::new(
            self.coords().x,
            self.coords().y,
            self.size().w,
            self.size().h,
        )
    }
}
