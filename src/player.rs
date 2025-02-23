use std::time::Instant;

use ggez::{
    graphics::{Canvas, Color, DrawParam, Image, Rect},
    mint::{Point2, Vector2},
    Context,
};

use crate::{
    consts::WINDOW_WIDTH,
    errors::DodgerError,
    utils::{validate_coordinates, RectSize},
};

#[derive(Clone)]
pub struct Player {
    pub coords: Point2<f32>,
    pub size: RectSize,
    pub scaling: Vector2<f32>,
    pub image: Image,
    pub blink_timer: Option<Instant>,
    pub alpha: f32,
}

impl Player {
    /// **Constructs and initializes a new `Player`.**
    ///
    /// ## Parameters
    /// * `_ctx`: the game context.
    /// * `coords`: initial coordinates of the player.
    /// * `scaling`: scaling factor to apply to the player's image.
    /// * `image`: the `Image` representing the player.
    ///
    /// ## Returns
    /// A result containing the new `Player`, or a `DodgerError` if coordinates validation fails.
    pub fn new(
        _ctx: &mut Context,
        coords: Point2<f32>,
        scaling: Vector2<f32>,
        image: &Image,
    ) -> Result<Self, DodgerError> {
        let validated_coords = validate_coordinates(coords)?;
        let w = image.width() as f32 * scaling.x;
        let h = image.width() as f32 * scaling.x;
        let size = RectSize::from((w, h));

        Ok(Player {
            coords: validated_coords,
            size,
            scaling,
            image: image.clone(),
            blink_timer: None,
            alpha: 0.0,
        })
    }

    /// **Moves the player to the left by a fixed amount.**
    ///
    /// ## Behavior
    /// Decreases the x-coordinate of the player by 20.0, ensuring it doesn't go below 0.0.
    pub fn move_left(&mut self) {
        self.coords.x -= 20.0_f32.max(0.0);
    }

    /// **Moves the player to the right by a fixed amount.**
    ///
    /// ## Behavior
    /// Increases the x-coordinate of the player by 20.0, ensuring the player doesn't go beyond the window boundaries.
    pub fn move_right(&mut self) {
        self.coords.x += 20.0_f32.min(WINDOW_WIDTH - self.size.w);
    }

    /// **Draws the player.**
    ///
    /// ## Parameters
    /// `canvas`: canvas to draw the player on.
    ///
    /// ## Behavior
    /// * Draws the player at the current position (`coords`) with a scaling factor (`scaling`).
    /// * Handles blinking effects if `blink_timer` is active:
    ///   * The alpha transparency of the player oscillates based on the elapsed time to create a blinking effect.
    pub fn draw(&mut self, canvas: &mut Canvas) {
        let mut draw_params = DrawParam::default().dest(self.coords).scale(self.scaling);

        if let Some(timer) = self.blink_timer {
            let elapsed = timer.elapsed().as_secs_f32();
            let blink_speed = 10.0; // Blinking frequency (times per second)

            self.alpha = (elapsed * blink_speed * std::f32::consts::PI).sin().abs();
            draw_params = draw_params.color(Color::new(1.0, 1.0, 1.0, self.alpha));
        }

        canvas.draw(&self.image, draw_params)
    }

    /// **Calculates the rectangular area occupied by the player.**
    ///
    /// ## Returns
    /// A `Rect` representing the boundaries of the player based on its position and size on the screen.
    pub fn rect(&self) -> Rect {
        Rect::new(self.coords.x, self.coords.y, self.size.w, self.size.h)
    }
}
