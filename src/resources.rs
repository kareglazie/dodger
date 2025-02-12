use ggez::{graphics::Image, Context};

use crate::levels::{get_levels, Level};

pub struct Resources {
    pub player_image: Image,
    pub background_image: Image,
    pub bad_object_image: Image,
    pub good_object_image: Image,
    pub level: Level,
}

impl Resources {
    fn load_image(ctx: &mut Context, path: &str) -> Image {
        Image::new(ctx, path).unwrap()
    }

    fn formatted_image_path(template: &str, image_type: &str) -> String {
        format!("{}/{}", template, image_type)
    }

    pub fn load_level(ctx: &mut Context, index: usize) -> Self {
        let levels = get_levels();
        let level = &levels[index];

        let player_path = Self::formatted_image_path(level.image_template, "player.png");
        let background_path = Self::formatted_image_path(level.image_template, "background.png");
        let bad_object_path = Self::formatted_image_path(level.image_template, "bad_object.png");
        let good_object_path = Self::formatted_image_path(level.image_template, "good_object.png");

        Resources {
            player_image: Self::load_image(ctx, &player_path),
            bad_object_image: Self::load_image(ctx, &bad_object_path),
            good_object_image: Self::load_image(ctx, &good_object_path),
            background_image: Self::load_image(ctx, &background_path),
            level: level.clone(),
        }
    }
}
