use crate::{
    buttons::{DrawText, IconButton, TextButton},
    consts::{WINDOW_HEIGHT, WINDOW_WIDTH},
    errors::DrawError,
    utils::validate_coordinates,
};
use ggez::{
    graphics::{
        draw, draw_queued_text, queue_text, Color, DrawMode, DrawParam, FilterMode, Font, Image,
        Mesh, Rect, Text,
    },
    mint::{Point2, Vector2},
    Context,
};

pub fn draw_background(ctx: &mut Context, image: &Image) -> Result<(), DrawError> {
    let scale_x = WINDOW_WIDTH / image.width() as f32;
    let scale_y = WINDOW_HEIGHT / image.height() as f32;

    draw(
        ctx,
        image,
        DrawParam::default()
            .dest(Point2 { x: 0.0, y: 0.0 })
            .scale(Vector2 {
                x: scale_x,
                y: scale_y,
            }),
    )
    .map_err(|err| DrawError::DrawBackground(err.to_string()))
}

pub fn draw_text(ctx: &mut Context, text_params: DrawText, font: Font) -> Result<(), DrawError> {
    let validated_coords = validate_coordinates(text_params.coords)?;
    let text = Text::new((text_params.text.clone(), font, text_params.size));
    draw(ctx, &text, (validated_coords, text_params.color))
        .map_err(|err| DrawError::DrawText(text_params.text, err.to_string()))
}

pub fn draw_timer(ctx: &mut Context, remaining_time: u64, font: Font) -> Result<(), DrawError> {
    let timer_text = Text::new((
        (if remaining_time < 10 {
            format!("00:0{}", remaining_time)
        } else {
            format!("00:{}", remaining_time)
        }),
        font,
        32.0,
    ));
    let coords = validate_coordinates(Point2 { x: 290.0, y: 10.0 })?;
    draw(ctx, &timer_text, (coords, Color::from_rgb(80, 80, 80)))
        .map_err(|err| DrawError::DrawTimer(err.to_string()))
}

pub fn draw_score(ctx: &mut Context, text_to_draw: DrawText, font: Font) -> Result<(), DrawError> {
    let coords = validate_coordinates(text_to_draw.coords)?;
    let score = text_to_draw.text.clone();
    let level_score_display = Text::new((text_to_draw.text, font, text_to_draw.size));
    queue_text(
        ctx,
        &level_score_display,
        [coords.x, coords.y],
        Some(text_to_draw.color),
    );

    draw_queued_text(ctx, DrawParam::default(), None, FilterMode::Linear)
        .map_err(|err| DrawError::DrawScore(score, err.to_string()))
}

pub fn draw_icon(ctx: &mut Context, icon_button: &IconButton) -> Result<(), DrawError> {
    let coords = validate_coordinates(icon_button.coords)?;
    let draw_params = DrawParam::default().dest(coords).scale(icon_button.scaling);

    draw(ctx, &icon_button.icon, draw_params)
        .map_err(|err| DrawError::DrawIconButton(err.to_string()))
}

pub fn draw_button_with_text(
    ctx: &mut Context,
    text_button: TextButton,
    font: Font,
) -> Result<(), DrawError> {
    let button_coords = validate_coordinates(text_button.coords)?;
    let button_rect = Rect::new(
        button_coords.x,
        button_coords.y,
        text_button.size.w,
        text_button.size.h,
    );
    let new_rect =
        &Mesh::new_rectangle(ctx, DrawMode::fill(), button_rect, text_button.button_color)
            .map_err(|err| DrawError::BuildRect(err.to_string()))?;

    draw(ctx, new_rect, DrawParam::default())
        .map_err(|err| DrawError::DrawRect(err.to_string()))?;

    let button_text = Text::new((text_button.text, font, text_button.text_size));
    let text_width = button_text.width(ctx);
    let text_height = button_text.height(ctx);

    let text_x = button_rect.x + (button_rect.w - text_width) / 2.0;
    let text_y = button_rect.y + (button_rect.h - text_height) / 2.0;

    let text_coords = validate_coordinates(Point2 {
        x: text_x,
        y: text_y,
    })?;

    draw(ctx, &button_text, (text_coords, text_button.text_color))
        .map_err(|err| DrawError::DrawTextButton(err.to_string()))
}
