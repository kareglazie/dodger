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
