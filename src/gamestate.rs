use std::time::{Duration, Instant};

use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::{Color, Rect};
use ggez::{graphics, Context, GameError, GameResult};
use rand::Rng;

use crate::buttons::{Coordinates, DrawText, IconButton, RectSize, TextButton};
use crate::levels::get_levels;
use crate::objects::FallingObject;
use crate::player::Player;
use crate::resources::Resources;
use crate::sound::AudioManager;
use crate::ui::{
    draw_background, draw_button_with_text, draw_icon, draw_score, draw_text, draw_timer,
    is_button_clicked,
};

pub struct GameState {
    total_score: i32,
    level_score: i32,
    level: usize,
    resources: Resources,
    player: Player,
    falling_objects: Vec<FallingObject>,
    last_update: Instant,
    level_start_time: Instant,
    game_over: bool,
    levels_completed: bool,
    audio: AudioManager,
    audio_button: IconButton,
    next_level_button: TextButton,
    restart_button: TextButton,
}

impl GameState {
    pub fn new(
        ctx: &mut Context,
        resources: Resources,
        level: usize,
        audio_manager: AudioManager,
    ) -> GameResult<Self> {
        let player = Player::new(ctx, 400.0, 520.0, &resources.player_image);
        let restart_button = TextButton::new(
            Coordinates::from((330.0, 350.0)),
            RectSize::from((140.0, 40.0)),
            "Restart".to_string(),
            30.0,
            Color::from_rgb(100, 100, 100),
            Color::WHITE,
        );

        let audio_button = IconButton::new(
            Coordinates::from((graphics::drawable_size(ctx).0 - 50.0, 10.0)),
            Coordinates::from((0.08, 0.08)),
            audio_manager.speaker_icon,
        );

        let next_level_button = TextButton::new(
            Coordinates::from((330.0, 350.0)),
            RectSize::from((140.0, 40.0)),
            "Next Level".to_string(),
            24.0,
            Color::WHITE,
            Color::from_rgb(100, 100, 100),
        );

        let s = GameState {
            total_score: 0,
            level_score: 0,
            player,
            level,
            resources,
            falling_objects: Vec::new(),
            last_update: Instant::now(),
            game_over: false,
            levels_completed: false,
            audio: AudioManager::new(ctx),
            level_start_time: Instant::now(),
            audio_button,
            next_level_button,
            restart_button,
        };
        Ok(s)
    }

    fn create_falling_object(&mut self) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0..780.0); // Случайная горизонтальная позиция
        let is_good = self.falling_objects.len() % 5 != 0;
        let object = FallingObject::new(x, 0.0, is_good, &self.resources);
        self.falling_objects.push(object);
    }

    fn handle_collisions(&mut self, ctx: &mut Context) {
        let player_rect = self.player.rect();

        let mut objects_to_remove = Vec::new();

        for (idx, obj) in self.falling_objects.iter().enumerate() {
            let obj_rect = obj.rect();

            if player_rect.overlaps(&obj_rect) {
                if obj.is_good {
                    self.audio.on_good_collision(ctx);
                    objects_to_remove.push((idx, true));
                } else {
                    self.audio.on_bad_collision(ctx);
                    self.game_over = true;
                    return;
                }
            }
        }

        for (idx, is_good) in objects_to_remove {
            if is_good {
                self.level_score += 10;
            }
            self.falling_objects.remove(idx);
        }
    }

    fn reset(&mut self, player: &Player) {
        self.player = player.clone();
        self.falling_objects.clear();
        if self.game_over {
            self.total_score = 0;
            self.level_score = 0;
        } else {
            self.total_score += self.level_score;
            self.level_score = 0;
        }
        self.game_over = false;
        self.level_start_time = Instant::now();
    }

    pub fn next_level(&mut self, player: &Player) -> GameResult<()> {
        self.falling_objects.clear(); // Очищаем падающие объекты
        self.last_update = Instant::now(); // Сбрасываем таймер
        self.player = player.clone();

        Ok(())
    }

    fn get_remaining_time(&self) -> u64 {
        let elapsed = self.level_start_time.elapsed();
        let remaining = Duration::from_secs(10).saturating_sub(elapsed);
        remaining.as_secs()
    }
}

impl EventHandler<GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let (width, height) = (
            self.audio_button.icon.width() as f32 * self.audio_button.coords_scale.x,
            self.audio_button.icon.height() as f32 * self.audio_button.coords_scale.y,
        );

        if is_button_clicked(
            ctx,
            Rect::new(
                self.audio_button.coords_dest.x,
                self.audio_button.coords_dest.y,
                width,
                height,
            ),
        ) {
            self.audio.is_muted = !self.audio.is_muted;
        }

        if !self.game_over && !self.levels_completed {
            if self.level_start_time.elapsed() >= Duration::from_secs(10) {
                let levels = get_levels();

                if self.level + 1 < levels.len() {
                    if is_button_clicked(
                        ctx,
                        Rect::new(
                            self.next_level_button.coords_dest.x,
                            self.next_level_button.coords_dest.y,
                            self.next_level_button.size.width,
                            self.next_level_button.size.height,
                        ),
                    ) {
                        self.level += 1;
                        self.resources = Resources::load_level(ctx, self.level);
                        let player = Player::new(ctx, 400.0, 520.0, &self.resources.player_image);
                        self.next_level(&player)?;
                        self.reset(&player);
                    }
                } else {
                    self.levels_completed = true;
                }
            } else {
                // Обычная логика игры
                if self.last_update.elapsed() >= Duration::from_millis(1000) {
                    self.last_update = Instant::now();
                    self.create_falling_object();
                }

                for obj in self.falling_objects.iter_mut() {
                    obj.update(&self.resources);
                }

                self.handle_collisions(ctx);
            }
        } else if self.game_over && is_button_clicked(
                ctx,
                Rect::new(
                    self.restart_button.coords_dest.x,
                    self.restart_button.coords_dest.y,
                    self.restart_button.size.width,
                    self.restart_button.size.height,
                ),
            )
            {
                self.reset(&self.player.clone());
            }
        

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::from_rgb(0, 0, 0));

        draw_background(ctx, &self.resources.background_image)?;

        let text = format!("Level: {}", self.level + 1);
        let text_to_draw = DrawText::new(Coordinates::from((300.0, 10.0)), text, 24.0, Color::WHITE);

        draw_text(ctx, text_to_draw)?;

        draw_timer(ctx, self.get_remaining_time())?;

        self.player.draw(ctx)?;

        for obj in &self.falling_objects {
            obj.draw(ctx)?;
        }

        let level_score_text = format!("Level Score: {}", self.level_score);
        let level_score_text_to_draw =
            DrawText::new(Coordinates::from((10.0, 10.0)), level_score_text, 24.0, Color::WHITE);
        draw_score(ctx, level_score_text_to_draw)?;

        let total_score_text = format!("Level Score: {}", self.level_score + self.total_score);
        let total_score_text_to_draw =
            DrawText::new(Coordinates::from((10.0, 10.0)), total_score_text, 24.0, Color::WHITE);
        draw_score(ctx, total_score_text_to_draw)?;

        let game_over_text =
            DrawText::new(Coordinates::from((300.0, 240.0)), "Game Over".to_string(), 48.0, Color::WHITE);
        if self.game_over {
            draw_text(ctx, game_over_text)?;

            draw_button_with_text(ctx, self.restart_button.clone())?;
        }

        if self.levels_completed {
            let game_complete_text = DrawText::new(
                Coordinates::from((200.0, 240.0)),
                "You Win! Game Over".to_string(),
                48.0,
                Color::WHITE,
            );
            draw_text(ctx, game_complete_text)?;

            let final_score_text = format!("Final Score: {}", self.total_score + self.level_score);
            let final_score_text_to_draw =
                DrawText::new(Coordinates::from((250.0, 300.0)), final_score_text, 32.0, Color::WHITE);
            draw_text(ctx, final_score_text_to_draw)?;
        }

        if self.level_start_time.elapsed() >= Duration::from_secs(10)
            && !self.game_over
            && !self.levels_completed
        {
            let level_complete_text = DrawText::new(
                Coordinates::from((250.0, 240.0)),
                "Level Complete".to_string(),
                48.0,
                Color::WHITE,
            );
            draw_text(ctx, level_complete_text)?;

            draw_button_with_text(ctx, self.next_level_button.clone())?;
        }

        if self.audio.is_muted {
            self.audio_button.icon = self.audio.speaker_muted_icon.clone();
        } else {
            self.audio_button.icon = self.audio.speaker_icon.clone();
        };
        draw_icon(ctx, &self.audio_button)?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if !self.game_over && !self.levels_completed {
            match keycode {
                KeyCode::Left => {
                    if self.player.x > 0.0 {
                        self.player.move_left();
                    }
                }
                KeyCode::Right => {
                    let screen_width = graphics::drawable_size(ctx).0;
                    if self.player.x < screen_width - self.player.width {
                        self.player.move_right(ctx);
                    }
                }
                _ => (),
            }
        }
    }
}
