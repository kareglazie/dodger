use ggez::{
    graphics::{FontData, Image},
    Context,
};

use crate::{errors::DodgerError, levels::Level};

/// **Adds custom fonts to the `Context`.**
///
/// ## Parameters
/// `ctx`: the game context.
///
/// ## Behavior
/// * Loads specific font files and adds them to the graphics context:
///   * `button_font` for UI elements like buttons.
///   * `text_font` for displaying pieces of text.
/// * If a font path is invalid, an error is returned.
pub fn add_fonts(ctx: &mut Context) -> Result<(), DodgerError> {
    let button_font_path = "/Fonts/button_font.otf";
    let text_font_path = "/Fonts/text_font.ttf";

    ctx.gfx.add_font(
        "button_font",
        FontData::from_path(ctx, button_font_path)
            .map_err(|_| DodgerError::InvalidFontPath(button_font_path.to_string()))?,
    );
    ctx.gfx.add_font(
        "text_font",
        FontData::from_path(ctx, text_font_path)
            .map_err(|_| DodgerError::InvalidFontPath(text_font_path.to_string()))?,
    );

    Ok(())
}

/// Structure for managing and storing game resources such as images and levels.
pub struct Resources {
    pub player_image: Image,
    pub background_image: Image,
    pub menu_background_image: Image,
    pub bad_object_image: Image,
    pub good_object_high_image: Image,
    pub good_object_medium_image: Image,
    pub good_object_low_image: Image,
    pub pause_button_image: Image,
    pub level: Level,
}

impl Resources {
    /// **Loads an image from a specific path.**
    ///
    /// ## Parameters
    /// * `ctx`: the game context.
    /// * `path`: relative path to the image file.
    ///
    /// ## Returns
    /// A result containing the `Image`, or a `DodgerError` if the image file is invalid or loading fails.
    fn load_image(ctx: &mut Context, path: &str) -> Result<Image, DodgerError> {
        Image::from_path(ctx, path).map_err(|_| DodgerError::InvalidImagePath(path.to_string()))
    }

    /// **Constructs a formatted image path based on a template and image type.**
    ///
    /// ## Parameters
    /// * `template`: a string representing folder for the images.
    /// * `image_type`: a string identifying the type of image (e.g., "player.png").
    ///
    /// ## Returns
    /// A formatted string combining the template and the image type.
    fn formatted_image_path(template: &str, image_type: &str) -> String {
        format!("{}/{}", template, image_type)
    }

    /// **Loads all resources for a specific game level.**
    ///
    /// ## Parameters
    /// * `ctx`: the game context.
    /// * `index`: index of the level (0-based).
    /// * `levels`: list of available levels.
    ///
    /// ## Behavior
    /// * Retrieves the `Level` at the specified index.
    /// * Loads all required resources for the level.
    ///
    /// ## Returns
    /// A result with `Resources`, or a `DodgerError` if there is an issue with paths or loading any resource.
    pub fn load_level(
        ctx: &mut Context,
        index: usize,
        levels: &[Level],
    ) -> Result<Self, DodgerError> {
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
            level: level.clone(),
        })
    }
}
