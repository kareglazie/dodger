use crate::{
    buttons::{DrawText, IconButton, TextButton},
    consts::{TEXT_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH, YELLOW},
    errors::DodgerError,
    utils::{text_button_rect, validate_coordinates, RectSize},
};
use ggez::{
    graphics::{Canvas, Color, DrawMode, DrawParam, Drawable, Image, Mesh},
    mint::{Point2, Vector2},
    Context,
};

/// **Draws a background image on the canvas, scaling it to fit the window dimensions.**
///
/// ## Parameters
/// * `canvas`: canvas to draw background on.
/// * `image`: image to use as a background.
///
/// ## Behavior
/// The image is scaled to fit the window width and height.
pub fn draw_background(canvas: &mut Canvas, image: &Image) {
    let scale_x = WINDOW_WIDTH / image.width() as f32;
    let scale_y = WINDOW_HEIGHT / image.height() as f32;

    canvas.draw(
        image,
        DrawParam::default()
            .dest(Point2 { x: 0.0, y: 0.0 })
            .scale(Vector2 {
                x: scale_x,
                y: scale_y,
            }),
    )
}

/// **Draws text on the canvas at the specified coordinates.**
///
/// ## Parameters
/// * `canvas`: canvas to draw the text on.
/// * `text`: `DrawText` struct containing text and its properties.
///
/// ## Returns
/// `Ok(())` if the text is drawn successfully, or a `DodgerError` if the coordinates are invalid.
///
/// ## Behavior
/// The text is drawn at the validated coordinates provided in the `DrawText` struct.
pub fn draw_text(canvas: &mut Canvas, text: DrawText) -> Result<(), DodgerError> {
    let validated_coords = validate_coordinates(text.coords)?;
    canvas.draw(&text.text, DrawParam::default().dest(validated_coords));
    Ok(())
}

/// **Draws a timer on the canvas, formatted as "00:SS".**
///
/// ## Parameters
/// * `ctx`: the game context.
/// * `canvas`: canvas to draw the timer on.
/// * `remaining_time`: the remaining time in seconds.
///
/// ## Returns
/// `Ok(())` if the timer is drawn successfully, or a `DodgerError` if the ellipse or text cannot be drawn.
///
/// ## Behavior
/// The timer is displayed as an ellipse with the remaining time centered inside it.
pub fn draw_timer(
    ctx: &mut Context,
    canvas: &mut Canvas,
    remaining_time: u64,
) -> Result<(), DodgerError> {
    let time = if remaining_time < 10 {
        format!("00:0{}", remaining_time)
    } else {
        format!("00:{}", remaining_time)
    };

    let timer_button = TextButton::new(
        Point2 { x: 360.0, y: 30.0 },
        YELLOW,
        RectSize::from((100.0, 50.0)),
        time,
        Color::BLACK,
        TEXT_SIZE,
        "text_font".to_string(),
    )?;

    draw_ellipse_with_text(ctx, canvas, timer_button)
}

/// **Draws score on the canvas at the specified coordinates.**
///
/// ## Parameters
/// * `canvas`: canvas to draw the score on.
/// * `score`: `DrawText` struct containing the score and its properties.
///
/// ## Returns
/// `Ok(())` if the score is drawn successfully, or a `DodgerError` if the coordinates are invalid.
///
/// ## Behavior
/// The score is drawn at the validated coordinates provided in the `DrawText` struct.
pub fn draw_score(canvas: &mut Canvas, score: DrawText) -> Result<(), DodgerError> {
    let coords = validate_coordinates(score.coords)?;
    let draw_params = DrawParam::default().dest(coords);

    canvas.draw(&score.text, draw_params);
    Ok(())
}

/// **Draws an icon on the canvas at the specified coordinates.**
///
/// ## Parameters
/// * `canvas`: canvas to draw the icon on.
/// * `icon_button`: `IconButton` struct containing the icon and its properties.
///
/// ## Returns
/// `Ok(())` if the icon is drawn successfully, or a `DodgerError` if the coordinates are invalid.
///
/// ##s Behavior
/// The icon is drawn at the validated coordinates and scaled according to the `IconButton` properties.
pub fn draw_icon(canvas: &mut Canvas, icon_button: &IconButton) -> Result<(), DodgerError> {
    let coords = validate_coordinates(icon_button.coords)?;
    let draw_params = DrawParam::default().dest(coords).scale(icon_button.scaling);

    canvas.draw(&icon_button.icon, draw_params);
    Ok(())
}

/// **Draws a button with text centered inside it.**
///
/// ## Parameters
/// * `ctx`: the game context.
/// * `canvas`: canvas to draw the button on.
/// * `text_button`: `TextButton` struct containing the button and text properties.
///
/// ## Returns
/// `Ok(())` if the button and text are drawn successfully, or a `DodgerError` if the rectangle or text cannot be drawn.
///
/// ## Behavior
/// The button is drawn as a rectangle, and the text is centered inside it.
pub fn draw_button_with_text(
    ctx: &mut Context,
    canvas: &mut Canvas,
    text_button: TextButton,
) -> Result<(), DodgerError> {
    let button_rect = text_button_rect(&text_button)?;

    let new_rect = Mesh::new_rectangle(
        &ctx.gfx,
        DrawMode::fill(),
        button_rect,
        text_button.button_color,
    )
    .map_err(|err| DodgerError::BuildRect(err.to_string()))?;

    canvas.draw(&new_rect, DrawParam::default());

    if let Some(text_size) = text_button.text.dimensions(ctx) {
        let text_width = text_size.w;
        let text_height = text_size.h;

        let text_x = button_rect.x + (button_rect.w - text_width) / 2.0;
        let text_y = button_rect.y + (button_rect.h - text_height) / 2.0;

        canvas.draw(
            &text_button.text,
            DrawParam::default().dest(Point2 {
                x: text_x,
                y: text_y,
            }),
        );
    }

    Ok(())
}

/// **Draws an ellipse with text centered inside it.**
///
/// ## Parameters
/// * `ctx`: the game context.
/// * `canvas`: canvas to draw the ellipse on.
/// * `button`: `TextButton` struct containing the ellipse and text properties.
///
/// ## Returns
/// `Ok(())` if the ellipse and text are drawn successfully, or a `DodgerError` if the ellipse or text cannot be drawn.
///
/// ## Behavior
/// The ellipse is drawn, and the text is centered inside it.
pub fn draw_ellipse_with_text(
    ctx: &mut Context,
    canvas: &mut Canvas,
    button: TextButton,
) -> Result<(), DodgerError> {
    let ellipse = Mesh::new_ellipse(
        &ctx.gfx,
        DrawMode::fill(),
        Point2::from_slice(&[button.coords.x, button.coords.y]),
        button.button_size.w / 2.0,
        button.button_size.h / 2.0,
        0.1,
        button.button_color,
    )
    .map_err(|err| DodgerError::BuildEllipse(err.to_string()))?;

    canvas.draw(&ellipse, DrawParam::default());

    if let Some(text_size) = button.text.dimensions(ctx) {
        let text_width = text_size.w;
        let text_height = text_size.h;

        let text_x = button.coords.x - (text_width / 2.0);
        let text_y = button.coords.y - (text_height / 2.0);

        canvas.draw(
            &button.text,
            DrawParam::default().dest(Point2 {
                x: text_x,
                y: text_y,
            }),
        );
    }
    Ok(())
}
