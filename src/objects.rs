use ggez::{
    graphics::Image,
    mint::{Point2, Vector2},
};

use crate::{
    resources::Resources,
    utils::{DrawableObject, RectSize},
};

pub struct FallingObject {
    pub coords: Point2<f32>,
    pub size: RectSize,
    pub scaling: Vector2<f32>,
    pub image: Image,
    pub is_good: bool,
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
            }
        }
    }

    pub fn update(&mut self, resources: &Resources) {
        let speed = resources.level.fall_speed;
        self.coords.y += 5.0 * speed * 0.5;
    }
}

impl DrawableObject for FallingObject {
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
