use ggez::{
    graphics::{Font, Image},
    Context, GameResult,
};

use crate::levels::Level;

pub struct Fonts {
    pub level_font: Font,
    pub lives_font: Font,
    pub score_font: Font,
}

impl Fonts {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let level_font = Font::new(ctx, "/Fonts/aloevera.ttf")?;
        let lives_font = Font::new(ctx, "/Fonts/sparkystones.ttf")?;
        let score_font = Font::new(ctx, "/Fonts/supercharge.otf")?;
        Ok(Self {
            level_font,
            lives_font,
            score_font,
        })
    }
}
pub struct Resources {
    pub player_image: Image,
    pub background_image: Image,
    pub bad_object_image: Image,
    pub good_object_high_image: Image,
    pub good_object_medium_image: Image,
    pub good_object_low_image: Image,
    pub fonts: Fonts,
    pub level: Level,
}

impl Resources {
    fn load_image(ctx: &mut Context, path: &str) -> Image {
        Image::new(ctx, path).unwrap()
    }

    fn formatted_image_path(template: &str, image_type: &str) -> String {
        format!("{}/{}", template, image_type)
    }

    pub fn load_level(ctx: &mut Context, index: usize, levels: &[Level]) -> GameResult<Self> {
        let level = &levels[index];

        let player_path = Self::formatted_image_path(level.image_template, "player.png");
        let background_path = Self::formatted_image_path(level.image_template, "background.png");
        let bad_object_path = Self::formatted_image_path(level.image_template, "bad_object.png");
        let good_object_high_path =
            Self::formatted_image_path(level.image_template, "/Good_Objects/high.png");
        let good_object_medium_path =
            Self::formatted_image_path(level.image_template, "/Good_Objects/medium.png");
        let good_object_low_path =
            Self::formatted_image_path(level.image_template, "/Good_Objects/low.png");
        let fonts = Fonts::new(ctx)?;

        Ok(Resources {
            player_image: Self::load_image(ctx, &player_path),
            bad_object_image: Self::load_image(ctx, &bad_object_path),
            good_object_high_image: Self::load_image(ctx, &good_object_high_path),
            good_object_medium_image: Self::load_image(ctx, &good_object_medium_path),
            good_object_low_image: Self::load_image(ctx, &good_object_low_path),
            background_image: Self::load_image(ctx, &background_path),
            fonts,
            level: level.clone(),
        })
    }
}
