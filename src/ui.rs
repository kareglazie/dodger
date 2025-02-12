use crate::buttons::{DrawText, IconButton, TextButton};
use ggez::graphics::{Color, DrawParam, Image, Mesh, Rect, Text};
use ggez::input::mouse::{button_pressed, position, MouseButton};
use ggez::mint::Point2;
use ggez::{graphics, Context, GameResult};

pub fn draw_background(ctx: &mut Context, image: &Image) -> GameResult<()> {
    let screen_size = graphics::drawable_size(ctx);
    let scale_x = screen_size.0 / image.width() as f32;
    let scale_y = screen_size.1 / image.height() as f32;

    graphics::draw(
        ctx,
        image,
        DrawParam::default()
            .dest(Point2 { x: 0.0, y: 0.0 })
            .scale(ggez::mint::Vector2 {
                x: scale_x,
                y: scale_y,
            }),
    )?;
    Ok(())
}

pub fn draw_text(ctx: &mut Context, text_params: DrawText) -> GameResult<()> {
    let text = graphics::Text::new((
        text_params.text,
        graphics::Font::default(),
        text_params.size,
    ));
    graphics::draw(
        ctx,
        &text,
        (
            ggez::mint::Point2 {
                x: text_params.coords_dest.x,
                y: text_params.coords_dest.y,
            },
            text_params.color,
        ),
    )?;
    Ok(())
}

pub fn draw_timer(ctx: &mut Context, remaining_time: u64) -> GameResult<()> {
    let timer_text = graphics::Text::new((
        (if remaining_time < 10 {
            format!("00:0{}", remaining_time)
        } else {
            format!("00:{}", remaining_time)
        }),
        graphics::Font::default(),
        24.0,
    ));
    graphics::draw(
        ctx,
        &timer_text,
        (ggez::mint::Point2 { x: 500.0, y: 10.0 }, Color::WHITE),
    )?;
    Ok(())
}

pub fn draw_score(ctx: &mut Context, text_to_draw: DrawText) -> GameResult<()> {
    let level_score_display = graphics::Text::new((
        text_to_draw.text,
        graphics::Font::default(),
        text_to_draw.size,
    ));
    graphics::queue_text(
        ctx,
        &level_score_display,
        [text_to_draw.coords_dest.x, text_to_draw.coords_dest.y],
        Some(text_to_draw.color),
    );

    graphics::draw_queued_text(
        ctx,
        graphics::DrawParam::default(),
        None,
        graphics::FilterMode::Linear,
    )?;
    Ok(())
}

pub fn draw_icon(ctx: &mut Context, icon_button: &IconButton) -> GameResult<()> {
    let icon_position = Point2 {
        x: icon_button.coords_dest.x,
        y: icon_button.coords_dest.y,
    };

    let draw_params =
        graphics::DrawParam::default()
            .dest(icon_position)
            .scale(ggez::mint::Vector2 {
                x: icon_button.coords_scale.x,
                y: icon_button.coords_scale.y,
            });

    graphics::draw(ctx, &icon_button.icon, draw_params)?;

    Ok(())
}

// pub fn construct_button_rect(x: f32, y: f32, width: f32, height: f32) -> GameResult<Rect> {
//     let button_rect = Rect::new(x, y, width, height);
//     Ok(button_rect)
// }

pub fn draw_button_with_text(ctx: &mut Context, text_button: TextButton) -> GameResult<()> {
    let button_rect = Rect::new(
        text_button.coords_dest.x,
        text_button.coords_dest.y,
        text_button.size.width,
        text_button.size.height,
    );
    let new_rect = &Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        button_rect,
        text_button.button_color,
    )?;

    graphics::draw(ctx, new_rect, DrawParam::default())?;

    let button_text = Text::new((
        text_button.text,
        graphics::Font::default(),
        text_button.text_size,
    ));
    let text_width = button_text.width(ctx);
    let text_height = button_text.height(ctx);
    let text_x = button_rect.x + (button_rect.w - text_width) / 2.0;
    let text_y = button_rect.y + (button_rect.h - text_height) / 2.0;

    graphics::draw(
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
