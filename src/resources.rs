use ggez::{
    graphics::{Font, Image},
    Context,
};

use crate::{errors::ResourceError, levels::Level};

pub struct Fonts {
    pub level_font: Font,
    pub lives_font: Font,
    pub score_font: Font,
}

impl Fonts {
    pub fn new(ctx: &mut Context) -> Result<Self, ResourceError> {
        let level_font_path = "/Fonts/aloevera.ttf";
        let lives_font_path = "/Fonts/sparkystones.ttf";
        let score_font_path = "/Fonts/supercharge.otf";
        let level_font = Font::new(ctx, level_font_path)
            .map_err(|_| ResourceError::InvalidFontPath(level_font_path.to_string()))?;
        let lives_font = Font::new(ctx, lives_font_path)
            .map_err(|_| ResourceError::InvalidFontPath(lives_font_path.to_string()))?;
        let score_font = Font::new(ctx, score_font_path)
            .map_err(|_| ResourceError::InvalidFontPath(score_font_path.to_string()))?;
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
    pub menu_background_image: Image,
    pub bad_object_image: Image,
    pub good_object_high_image: Image,
    pub good_object_medium_image: Image,
    pub good_object_low_image: Image,
    pub pause_button_image: Image,
    pub fonts: Fonts,
    pub level: Level,
}

impl Resources {
    fn load_image(ctx: &mut Context, path: &str) -> Result<Image, ResourceError> {
        Image::new(ctx, path).map_err(|_| ResourceError::InvalidImagePath(path.to_string()))
    }

    fn formatted_image_path(template: &str, image_type: &str) -> String {
        format!("{}/{}", template, image_type)
    }

    pub fn load_level(
        ctx: &mut Context,
        index: usize,
        levels: &[Level],
    ) -> Result<Self, ResourceError> {
        let level = &levels[index];

        let player_path = Self::formatted_image_path(level.image_template, "player.png");
        let background_path = Self::formatted_image_path(level.image_template, "background.png");
        let menu_background_path = "/menu_background.png".to_string();
        let pause_button_path: String = "/pause_resume.png".to_string();
        let bad_object_path = Self::formatted_image_path(level.image_template, "bad_object.png");
        let good_object_high_path =
            Self::formatted_image_path(level.image_template, "/Good_Objects/high.png");
        let good_object_medium_path =
            Self::formatted_image_path(level.image_template, "/Good_Objects/medium.png");
        let good_object_low_path =
            Self::formatted_image_path(level.image_template, "/Good_Objects/low.png");
        let fonts = Fonts::new(ctx)?;
        let player_image = Self::load_image(ctx, &player_path)?;
        let bad_object_image = Self::load_image(ctx, &bad_object_path)?;
        let good_object_high_image = Self::load_image(ctx, &good_object_high_path)?;
        let good_object_medium_image = Self::load_image(ctx, &good_object_medium_path)?;
        let good_object_low_image = Self::load_image(ctx, &good_object_low_path)?;
        let background_image = Self::load_image(ctx, &background_path)?;
        let menu_background_image = Self::load_image(ctx, &menu_background_path)?;
        let pause_button_image = Self::load_image(ctx, &pause_button_path)?;

        Ok(Resources {
            player_image,
            bad_object_image,
            good_object_high_image,
            good_object_medium_image,
            good_object_low_image,
            background_image,
            menu_background_image,
            pause_button_image,
            fonts,
            level: level.clone(),
        })
    }
}
