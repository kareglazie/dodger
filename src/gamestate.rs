use std::time::{Duration, Instant};

use ggez::{
    event::{EventHandler, KeyCode, KeyMods},
    graphics::{clear, drawable_size, present, Color, Rect},
    mint::{Point2, Vector2},
    Context, GameError, GameResult,
};
use rand::Rng;

use crate::{
    buttons::{DrawText, IconButton, TextButton},
    levels::{get_levels, Level},
    objects::FallingObject,
    player::Player,
    resources::Resources,
    sound::AudioManager,
    ui::{
        draw_background, draw_button_with_text, draw_icon, draw_score, draw_text, draw_timer,
        is_button_clicked,
    },
    utils::RectSize,
};

pub struct GameState {
    total_score: i32,
    level_score: i32,
    current_level: usize,
    levels: Vec<Level>,
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
    lives: i32,
    level_complete_sound_played: bool,
    victory_sound_played: bool,
}

impl GameState {
    pub fn new(
        ctx: &mut Context,
        resources: Resources,
        current_level: usize,
        audio_manager: AudioManager,
    ) -> GameResult<Self> {
        let player = Player::new(
            ctx,
            Point2::from_slice(&[400.0, 520.0]),
            Vector2::from_slice(&[0.2, 0.2]),
            &resources.player_image,
        );
        let restart_button = TextButton::new(
            Point2::from_slice(&[320.0, 350.0]),
            RectSize::from((140.0, 40.0)),
            "Restart".to_string(),
            30.0,
            Color::from_rgb(100, 100, 100),
            Color::WHITE,
        );

        let audio_button = IconButton::new(
            Point2::from_slice(&[drawable_size(ctx).0 - 50.0, 10.0]),
            Vector2::from_slice(&[0.08, 0.08]),
            audio_manager.speaker_icon,
        );

        let next_level_button = TextButton::new(
            Point2::from_slice(&[330.0, 350.0]),
            RectSize::from((140.0, 40.0)),
            "Next Level".to_string(),
            24.0,
            Color::WHITE,
            Color::from_rgb(100, 100, 100),
        );

        let levels = get_levels();

        let s = GameState {
            total_score: 0,
            level_score: 0,
            player,
            current_level,
            levels,
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
            lives: 1,
            level_complete_sound_played: false,
            victory_sound_played: false,
        };
        Ok(s)
    }

    fn create_falling_object(&mut self) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0..780.0); // Случайная горизонтальная позиция
        let is_good = self.falling_objects.len() % 5 != 0; // Каждый пятый объект - bad
        let object = FallingObject::new(
            Point2::from_slice(&[x, 0.0]),
            Vector2::from_slice(&[0.08, 0.08]),
            is_good,
            &self.resources,
        );
        self.falling_objects.push(object);
    }

    fn handle_collisions(&mut self, ctx: &mut Context) {
        let player_rect = self.player.rect();

        for obj in &mut self.falling_objects {
            if obj.remove_timer.is_some() {
                continue;
            }

            let obj_rect = obj.rect();

            if player_rect.overlaps(&obj_rect) {
                if obj.is_good {
                    self.audio.play_sound(ctx, "good_collision".to_string());
                    obj.remove_timer = Some(Instant::now()); // Удаляем "хороший" объект сразу
                } else {
                    self.audio.play_sound(ctx, "bad_collision".to_string());
                    self.lives -= 1;
                    if self.lives <= 0 {
                        self.game_over = true;
                        self.audio.play_sound(ctx, "game_over".to_string());
                    }
                    obj.remove_timer = Some(Instant::now()); // Запускаем таймер для удаления
                    obj.blink_timer = Some(Instant::now()); // Запускаем таймер для мигания "плохого" объекта
                    self.player.blink_timer = Some(Instant::now()); // Запускаем таймер для мигания игрока при столкновении с "плохим" объектом
                }
            }
        }

        // Удаляем объекты, у которых таймер истек
        self.falling_objects.retain(|obj| {
            if let Some(timer) = obj.remove_timer {
                if obj.is_good || timer.elapsed() >= Duration::from_millis(500) {
                    false // Удалить объект
                } else {
                    true // Оставить объект
                }
            } else {
                true // Оставить объект, если таймер не запущен
            }
        });
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
        self.levels_completed = false;
        self.lives = 5;
        self.level_complete_sound_played = false;
        self.victory_sound_played = false;
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
            self.audio_button.icon.width() as f32 * self.audio_button.scaling.x,
            self.audio_button.icon.height() as f32 * self.audio_button.scaling.y,
        );

        if is_button_clicked(
            ctx,
            Rect::new(
                self.audio_button.coords.x,
                self.audio_button.coords.y,
                width,
                height,
            ),
        ) {
            self.audio.is_muted = !self.audio.is_muted;
        }

        if !self.game_over {
            if !self.levels_completed {
                if self.level_start_time.elapsed() >= Duration::from_secs(10) {
                    if !self.level_complete_sound_played
                        && self.current_level + 1 < self.levels.len()
                    {
                        self.audio.play_sound(ctx, "level_completed".to_string());
                        self.level_complete_sound_played = true;
                    }

                    if self.current_level + 1 < self.levels.len() {
                        if is_button_clicked(
                            ctx,
                            Rect::new(
                                self.next_level_button.coords.x,
                                self.next_level_button.coords.y,
                                self.next_level_button.size.w,
                                self.next_level_button.size.h,
                            ),
                        ) {
                            self.current_level += 1;
                            self.resources = Resources::load_level(ctx, self.current_level)?;
                            let player = Player::new(
                                ctx,
                                Point2::from_slice(&[400.0, 520.0]),
                                Vector2::from_slice(&[0.2, 0.2]),
                                &self.resources.player_image,
                            );
                            self.next_level(&player)?;
                            self.reset(&player);
                        }
                    } else {
                        self.levels_completed = true;
                    }
                }
            } else if self.levels_completed {
                if !self.victory_sound_played {
                    self.audio.play_sound(ctx, "victory".to_string());
                    self.victory_sound_played = true;
                }
                if is_button_clicked(
                    ctx,
                    Rect::new(
                        self.restart_button.coords.x,
                        self.restart_button.coords.y,
                        self.restart_button.size.w,
                        self.restart_button.size.h,
                    ),
                ) {
                    self.reset(&self.player.clone());
                }
            }

            // Обычная логика игры
            if !self.levels_completed && self.last_update.elapsed() >= Duration::from_millis(1000) {
                self.last_update = Instant::now();
                self.create_falling_object();
            }

            for obj in &mut self.falling_objects {
                // Обновляем только объекты без активного таймера
                if obj.remove_timer.is_none() {
                    obj.update(&self.resources);
                }
            }

            self.handle_collisions(ctx);

            if let Some(timer) = self.player.blink_timer {
                if timer.elapsed() >= Duration::from_secs(1) {
                    self.player.blink_timer = None;
                    self.player.alpha = 1.0;
                }
            }
        } else if self.game_over
            && is_button_clicked(
                ctx,
                Rect::new(
                    self.restart_button.coords.x,
                    self.restart_button.coords.y,
                    self.restart_button.size.w,
                    self.restart_button.size.h,
                ),
            )
        {
            self.reset(&self.player.clone());
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, Color::from_rgb(0, 0, 0));

        draw_background(ctx, &self.resources.background_image)?;

        let text = format!("Level: {}", self.current_level + 1);
        let text_to_draw =
            DrawText::new(Point2::from_slice(&[300.0, 10.0]), text, 32.0, Color::WHITE);

        draw_text(ctx, text_to_draw, self.resources.fonts.level_font)?;

        draw_timer(
            ctx,
            self.get_remaining_time(),
            self.resources.fonts.level_font,
        )?;

        self.player.draw(ctx)?;

        for obj in &mut self.falling_objects {
            obj.draw(ctx)?;
        }

        let level_score_text = format!("Level Score: {}", self.level_score);
        let level_score_text_to_draw = DrawText::new(
            Point2::from_slice(&[10.0, 10.0]),
            level_score_text,
            24.0,
            Color::WHITE,
        );
        draw_score(
            ctx,
            level_score_text_to_draw,
            self.resources.fonts.score_font,
        )?;

        let total_score_text = format!("Total Score: {}", self.level_score + self.total_score);
        let total_score_text_to_draw = DrawText::new(
            Point2::from_slice(&[10.0, 50.0]),
            total_score_text,
            24.0,
            Color::WHITE,
        );
        draw_score(
            ctx,
            total_score_text_to_draw,
            self.resources.fonts.score_font,
        )?;

        let lives_text_to_draw = DrawText::new(
            Point2::from_slice(&[310.0, 50.0]),
            format!("Lives: {}", self.lives),
            24.0,
            Color::WHITE,
        );
        draw_text(ctx, lives_text_to_draw, self.resources.fonts.lives_font)?;

        let game_over_text = DrawText::new(
            Point2::from_slice(&[300.0, 240.0]),
            "Game Over".to_string(),
            48.0,
            Color::WHITE,
        );
        if self.game_over {
            draw_text(ctx, game_over_text, self.resources.fonts.score_font)?;

            draw_button_with_text(
                ctx,
                self.restart_button.clone(),
                self.resources.fonts.lives_font,
            )?;
        }

        if self.levels_completed {
            let game_complete_text = DrawText::new(
                Point2::from_slice(&[200.0, 240.0]),
                "You Win! Game Over".to_string(),
                48.0,
                Color::WHITE,
            );
            draw_text(ctx, game_complete_text, self.resources.fonts.level_font)?;

            let final_score_text = format!("Final Score: {}", self.total_score + self.level_score);
            let final_score_text_to_draw = DrawText::new(
                Point2::from_slice(&[300.0, 300.0]),
                final_score_text,
                32.0,
                Color::WHITE,
            );
            draw_text(
                ctx,
                final_score_text_to_draw,
                self.resources.fonts.lives_font,
            )?;
            draw_button_with_text(
                ctx,
                self.restart_button.clone(),
                self.resources.fonts.lives_font,
            )?;
        }

        if self.level_start_time.elapsed() >= Duration::from_secs(10)
            && !self.game_over
            && !self.levels_completed
        {
            let level_complete_text = DrawText::new(
                Point2::from_slice(&[250.0, 240.0]),
                "Level Complete".to_string(),
                48.0,
                Color::WHITE,
            );
            draw_text(ctx, level_complete_text, self.resources.fonts.level_font)?;

            draw_button_with_text(
                ctx,
                self.next_level_button.clone(),
                self.resources.fonts.level_font,
            )?;
        }

        if self.audio.is_muted {
            self.audio_button.icon = self.audio.speaker_muted_icon.clone();
        } else {
            self.audio_button.icon = self.audio.speaker_icon.clone();
        };
        draw_icon(ctx, &self.audio_button)?;

        present(ctx)?;
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
                    if self.player.coords.x > 0.0 {
                        self.player.move_left();
                    }
                }
                KeyCode::Right => {
                    let screen_width = drawable_size(ctx).0;
                    if self.player.coords.x < screen_width - self.player.size.w {
                        self.player.move_right(ctx);
                    }
                }
                _ => (),
            }
        }
    }
}
