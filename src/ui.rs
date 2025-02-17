use crate::buttons::{DrawText, IconButton, TextButton};
use ggez::{
    graphics::{
        draw, draw_queued_text, drawable_size, queue_text, Color, DrawMode, DrawParam, FilterMode,
        Font, Image, Mesh, Rect, Text,
    },
    input::mouse::{button_pressed, position, MouseButton},
    mint::{Point2, Vector2},
    Context, GameResult,
};

pub fn draw_background(ctx: &mut Context, image: &Image) -> GameResult<()> {
    let screen_size = drawable_size(ctx);
    let scale_x = screen_size.0 / image.width() as f32;
    let scale_y = screen_size.1 / image.height() as f32;

    draw(
        ctx,
        image,
        DrawParam::default()
            .dest(Point2 { x: 0.0, y: 0.0 })
            .scale(Vector2 {
                x: scale_x,
                y: scale_y,
            }),
    )?;
    Ok(())
}

pub fn draw_text(ctx: &mut Context, text_params: DrawText, font: Font) -> GameResult<()> {
    let text = Text::new((text_params.text, font, text_params.size));
    draw(ctx, &text, (text_params.coords, text_params.color))?;
    Ok(())
}

pub fn draw_timer(ctx: &mut Context, remaining_time: u64, font: Font) -> GameResult<()> {
    let timer_text = Text::new((
        (if remaining_time < 10 {
            format!("00:0{}", remaining_time)
        } else {
            format!("00:{}", remaining_time)
        }),
        font,
        24.0,
    ));
    draw(
        ctx,
        &timer_text,
        (Point2 { x: 500.0, y: 10.0 }, Color::WHITE),
    )?;
    Ok(())
}

pub fn draw_score(ctx: &mut Context, text_to_draw: DrawText, font: Font) -> GameResult<()> {
    let level_score_display = Text::new((text_to_draw.text, font, text_to_draw.size));
    queue_text(
        ctx,
        &level_score_display,
        [text_to_draw.coords.x, text_to_draw.coords.y],
        Some(text_to_draw.color),
    );

    draw_queued_text(ctx, DrawParam::default(), None, FilterMode::Linear)?;
    Ok(())
}

pub fn draw_icon(ctx: &mut Context, icon_button: &IconButton) -> GameResult<()> {
    let draw_params = DrawParam::default()
        .dest(icon_button.coords)
        .scale(icon_button.scaling);

    draw(ctx, &icon_button.icon, draw_params)?;

    Ok(())
}

pub fn draw_button_with_text(ctx: &mut Context, text_button: TextButton) -> GameResult<()> {
    let button_rect = Rect::new(
        text_button.coords.x,
        text_button.coords.y,
        text_button.size.w,
        text_button.size.h,
    );
    let new_rect =
        &Mesh::new_rectangle(ctx, DrawMode::fill(), button_rect, text_button.button_color)?;

    draw(ctx, new_rect, DrawParam::default())?;

    let button_text = Text::new((text_button.text, Font::default(), text_button.text_size));
    let text_width = button_text.width(ctx);
    let text_height = button_text.height(ctx);
    let text_x = button_rect.x + (button_rect.w - text_width) / 2.0;
    let text_y = button_rect.y + (button_rect.h - text_height) / 2.0;

    draw(
        ctx,
        &button_text,
        (
            ggez::mint::Point2 {
                x: text_x,
                y: text_y,
            },
            text_button.text_color,
        ),
    )?;

    Ok(())
}

pub fn is_button_clicked(ctx: &mut Context, button_rect: Rect) -> bool {
    if button_pressed(ctx, MouseButton::Left) {
        let pos = position(ctx);
        button_rect.contains(pos)
    } else {
        false
    }
}
