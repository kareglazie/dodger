use std::time::{Duration, Instant};

use ggez::{
    event::{quit, EventHandler, KeyCode, KeyMods},
    graphics::{clear, present, Color, Image, Rect},
    mint::{Point2, Vector2},
    Context, GameError, GameResult,
};
use rand::Rng;

use crate::{
    buttons::{DrawText, IconButton, TextButton},
    consts::{LEVEL_DURATION_SECS, LIVES, WINDOW_HEIGHT, WINDOW_WIDTH},
    errors::DrawError,
    levels::{get_levels, Level},
    modes::GameMode,
    objects::{FallingObject, GoodObjectValue},
    player::Player,
    resources::Resources,
    sound::AudioManager,
    ui::{
        draw_background, draw_button_with_text, draw_icon, draw_score, draw_text, draw_timer,
        get_level_button, is_button_clicked,
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
    paused_time: Option<Instant>,
    audio: AudioManager,
    audio_button: IconButton,
    start_button: TextButton,
    exit_button: TextButton,
    resume_button: TextButton,
    menu_button: TextButton,
    pause_button: IconButton,
    next_level_button: TextButton,
    restart_button: TextButton,
    select_level_button: TextButton,
    lives: u8,
    game_mode: GameMode,
    level_complete_sound_played: bool,
    victory_sound_played: bool,
    game_over_sound_played: bool,
    game_started: bool,
    is_paused: bool,
}

impl GameState {
    pub fn new(
        ctx: &mut Context,
        resources: Resources,
        current_level: usize,
        audio_manager: AudioManager,
    ) -> Result<Self, DrawError> {
        let player = Player::new(
            ctx,
            Point2::from_slice(&[WINDOW_WIDTH / 2.0, WINDOW_HEIGHT - 175.0]),
            Vector2::from_slice(&[0.35, 0.35]),
            &resources.player_image,
        )?;
        let restart_button = TextButton::new(
            Point2::from_slice(&[WINDOW_WIDTH / 2.0 - 65.0, WINDOW_HEIGHT / 2.0 - 30.0]),
            RectSize::from((140.0, 40.0)),
            "Restart".to_string(),
            30.0,
            Color::from_rgb(100, 100, 100),
            Color::WHITE,
        )?;

        let audio_button = IconButton::new(
            Point2::from_slice(&[WINDOW_WIDTH - 50.0, 60.0]),
            Vector2::from_slice(&[0.08, 0.08]),
            audio_manager.speaker_icon,
        )?;

        let pause_button = IconButton::new(
            Point2::from_slice(&[WINDOW_WIDTH / 2.0 - 40.0, WINDOW_HEIGHT / 2.0 - 30.0]),
            Vector2::from_slice(&[0.5, 0.5]),
            Image::new(ctx, "/pause_resume.png").unwrap(),
        )?;

        let next_level_button = TextButton::new(
            Point2::from_slice(&[WINDOW_WIDTH / 2.0 - 65.0, WINDOW_HEIGHT / 2.0 - 30.0]),
            RectSize::from((150.0, 60.0)),
            "Next Level".to_string(),
            28.0,
            Color::WHITE,
            Color::from_rgb(200, 200, 200),
        )?;

        let start_button = TextButton::new(
            Point2::from_slice(&[WINDOW_WIDTH / 2.0 - 80.0, WINDOW_HEIGHT / 2.0 - 80.0]),
            RectSize::from((200.0, 50.0)),
            "Start".to_string(),
            30.0,
            Color::from_rgb(100, 100, 100),
            Color::WHITE,
        )?;

        let resume_button = TextButton::new(
            Point2::from_slice(&[WINDOW_WIDTH / 2.0 - 80.0, WINDOW_HEIGHT / 2.0 - 80.0]),
            RectSize::from((200.0, 50.0)),
            "Resume".to_string(),
            30.0,
            Color::from_rgb(100, 100, 100),
            Color::WHITE,
        )?;

        let exit_button = TextButton::new(
            Point2::from_slice(&[WINDOW_WIDTH / 2.0 - 80.0, WINDOW_HEIGHT / 2.0 + 80.0]),
            RectSize::from((200.0, 50.0)),
            "Exit".to_string(),
            30.0,
            Color::from_rgb(100, 100, 100),
            Color::WHITE,
        )?;

        let select_level_button = TextButton::new(
            Point2::from_slice(&[WINDOW_WIDTH / 2.0 - 80.0, WINDOW_HEIGHT / 2.0]),
            RectSize::from((200.0, 50.0)),
            "Select Level".to_string(),
            30.0,
            Color::from_rgb(100, 100, 100),
            Color::WHITE,
        )?;

        let menu_button = TextButton::new(
            Point2::from_slice(&[WINDOW_WIDTH - 100.0, 10.0]),
            RectSize::from((100.0, 40.0)),
            "Menu".to_string(),
            24.0,
            Color::WHITE,
            Color::from_rgb(100, 100, 100),
        )?;

        let levels = get_levels();

        let audio = AudioManager::new(ctx)?;

        let game = GameState {
            total_score: 0,
            level_score: 0,
            player,
            current_level,
            levels,
            resources,
            falling_objects: Vec::new(),
            last_update: Instant::now(),
            paused_time: None,
            audio,
            level_start_time: Instant::now(),
            audio_button,
            start_button,
            resume_button,
            exit_button,
            menu_button,
            pause_button,
            next_level_button,
            restart_button,
            select_level_button,
            lives: LIVES,
            game_mode: GameMode::Menu,
            level_complete_sound_played: false,
            victory_sound_played: false,
            game_over_sound_played: false,
            game_started: false,
            is_paused: false,
        };
        Ok(game)
    }

    fn create_falling_object(&mut self) -> Result<(), DrawError> {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(25.0..WINDOW_WIDTH - 25.0); // Случайная горизонтальная позиция
        let is_good = self.falling_objects.len() % 5 != 0; // Каждый пятый объект - bad
        let good_object_value = if is_good {
            // Случайно выбираем значение для "хорошего" объекта
            match rng.gen_range(0..10) {
                0 => Some(GoodObjectValue::High),
                1 | 3 | 5 => Some(GoodObjectValue::Medium),
                _ => Some(GoodObjectValue::Low),
            }
        } else {
            None
        };

        let object = FallingObject::new(
            Point2::from_slice(&[x, 0.0]),
            Vector2::from_slice(&[0.08, 0.08]),
            is_good,
            good_object_value,
            &self.resources,
        )?;

        self.falling_objects.push(object);

        Ok(())
    }

    fn handle_collisions(&mut self, ctx: &mut Context) -> Result<(), DrawError> {
        let player_rect = self.player.rect();

        for obj in &mut self.falling_objects {
            if obj.remove_timer.is_some() {
                continue;
            }

            let obj_rect = obj.rect();

            if player_rect.overlaps(&obj_rect) {
                if obj.is_good {
                    match &obj.good_object_value {
                        Some(value) => {
                            self.level_score += value.score();
                            match value {
                                GoodObjectValue::High => {
                                    self.audio
                                        .play_sound(ctx, "good_collision_high".to_string())?;
                                }
                                _ => {
                                    self.audio.play_sound(ctx, "good_collision".to_string())?;
                                }
                            }
                        }
                        None => {
                            self.level_score += 10;
                            self.audio.play_sound(ctx, "good_collision".to_string())?;
                        }
                    }
                    obj.remove_timer = Some(Instant::now()); // Удаляем "хороший" объект сразу
                } else {
                    self.audio.play_sound(ctx, "bad_collision".to_string())?;
                    self.lives -= 1;
                    if self.lives == 0 {
                        self.game_mode = GameMode::GameOver;
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
                if obj.is_good || timer.elapsed() >= Duration::from_secs(1) {
                    false // Удалить объект
                } else {
                    true // Оставить объект
                }
            } else {
                true // Оставить объект, если таймер не запущен
            }
        });
        Ok(())
    }

    fn reset(&mut self, ctx: &mut Context) -> Result<(), DrawError> {
        if self.game_mode == GameMode::GameOver
            || self.game_mode == GameMode::Victory
            || self.game_mode == GameMode::LevelSelection
        {
            self.total_score = 0;
        } else {
            self.total_score += self.level_score;
        }
        self.falling_objects.clear();
        self.level_score = 0;
        self.lives = LIVES;
        self.level_start_time = Instant::now();
        self.last_update = Instant::now();
        self.level_complete_sound_played = false;
        self.victory_sound_played = false;
        self.game_over_sound_played = false;
        self.is_paused = false;
        self.resources = Resources::load_level(ctx, self.current_level, &self.levels)?;

        self.player = Player::new(
            ctx,
            Point2::from_slice(&[WINDOW_WIDTH / 2.0, WINDOW_HEIGHT - 175.0]),
            Vector2::from_slice(&[0.4, 0.4]),
            &self.resources.player_image,
        )?;

        self.game_mode = GameMode::Playing;

        Ok(())
    }

    fn pause(&mut self) {
        if self.game_mode == GameMode::Playing {
            self.paused_time = Some(Instant::now());
            self.is_paused = true;
        }
    }

    fn resume(&mut self) {
        if self.is_paused {
            if let Some(paused_time) = self.paused_time {
                let pause_duration = paused_time.elapsed();
                self.level_start_time += pause_duration;
                self.last_update += pause_duration;
            }
            self.paused_time = None;
            self.is_paused = false;
        }
    }

    fn get_remaining_time(&self) -> u64 {
        if self.is_paused {
            let elapsed = self.last_update.duration_since(self.level_start_time);
            let remaining = Duration::from_secs(LEVEL_DURATION_SECS).saturating_sub(elapsed);
            remaining.as_secs()
        } else {
            let elapsed = self.level_start_time.elapsed();
            let remaining = Duration::from_secs(LEVEL_DURATION_SECS).saturating_sub(elapsed);
            remaining.as_secs()
        }
    }

    fn update_menu(&mut self, ctx: &mut Context) -> Result<(), DrawError> {
        if is_button_clicked(
            ctx,
            Rect::new(
                self.start_button.coords.x,
                self.start_button.coords.y,
                self.start_button.size.w,
                self.start_button.size.h,
            ),
        ) {
            self.level_start_time = Instant::now();
            self.last_update = Instant::now();
            self.game_mode = GameMode::Playing;
        }

        if is_button_clicked(
            ctx,
            Rect::new(
                self.select_level_button.coords.x,
                self.select_level_button.coords.y,
                self.select_level_button.size.w,
                self.select_level_button.size.h,
            ),
        ) {
            self.game_mode = GameMode::LevelSelection;
        }

        if is_button_clicked(
            ctx,
            Rect::new(
                self.exit_button.coords.x,
                self.exit_button.coords.y,
                self.exit_button.size.w,
                self.exit_button.size.h,
            ),
        ) {
            quit(ctx);
        }
        Ok(())
    }

    fn draw_menu(&mut self, ctx: &mut Context) -> Result<(), DrawError> {
        draw_background(ctx, &self.resources.menu_background_image)?;
        if !self.game_started {
            draw_button_with_text(
                ctx,
                self.start_button.clone(),
                self.resources.fonts.lives_font,
            )?;
        } else {
            draw_button_with_text(
                ctx,
                self.resume_button.clone(),
                self.resources.fonts.lives_font,
            )?;
        }
        draw_button_with_text(
            ctx,
            self.exit_button.clone(),
            self.resources.fonts.lives_font,
        )?;
        draw_button_with_text(
            ctx,
            self.select_level_button.clone(),
            self.resources.fonts.lives_font,
        )?;
        Ok(())
    }

    fn update_playing(&mut self, ctx: &mut Context) -> Result<(), DrawError> {
        if !self.game_started {
            self.game_started = true;
        }

        if self.is_paused {
            return Ok(());
        }
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

        if is_button_clicked(
            ctx,
            Rect::new(
                self.menu_button.coords.x,
                self.menu_button.coords.y,
                self.menu_button.size.w,
                self.menu_button.size.h,
            ),
        ) {
            self.game_mode = GameMode::Menu;
        }

        if self.level_start_time.elapsed() >= Duration::from_secs(LEVEL_DURATION_SECS) {
            if self.current_level + 1 < self.levels.len() {
                self.game_mode = GameMode::NextLevel;
            } else {
                self.game_mode = GameMode::Victory;
            }
        }

        if self.last_update.elapsed() >= Duration::from_millis(800) {
            self.last_update = Instant::now();
            self.create_falling_object()?;
        }

        for obj in &mut self.falling_objects {
            if obj.remove_timer.is_none() {
                obj.update(&self.resources, 0.1);
            }
        }

        self.handle_collisions(ctx)?;

        if let Some(timer) = self.player.blink_timer {
            if timer.elapsed() >= Duration::from_secs(1) {
                self.player.blink_timer = None;
                self.player.alpha = 1.0;
            }
        }
        Ok(())
    }

    fn draw_playing(&mut self, ctx: &mut Context) -> Result<(), DrawError> {
        draw_background(ctx, &self.resources.background_image)?;
        self.player.draw(ctx)?;
        draw_button_with_text(
            ctx,
            self.menu_button.clone(),
            self.resources.fonts.lives_font,
        )?;

        let text = format!("Level: {}", self.current_level + 1);
        let text_to_draw = DrawText::new(
            Point2::from_slice(&[525.0, 10.0]),
            text,
            32.0,
            Color::from_rgb(80, 80, 80),
        )?;

        draw_text(ctx, text_to_draw, self.resources.fonts.level_font)?;
        if self.audio.is_muted {
            self.audio_button.icon = self.audio.speaker_muted_icon.clone();
        } else {
            self.audio_button.icon = self.audio.speaker_icon.clone();
        };
        draw_icon(ctx, &self.audio_button)?;
        for obj in &mut self.falling_objects {
            obj.draw(ctx)?;
        }

        let level_score_text = format!("Level Score: {}", self.level_score);
        let level_score_text_to_draw = DrawText::new(
            Point2::from_slice(&[10.0, 10.0]),
            level_score_text,
            30.0,
            Color::WHITE,
        )?;
        draw_score(
            ctx,
            level_score_text_to_draw,
            self.resources.fonts.score_font,
        )?;

        draw_timer(
            ctx,
            self.get_remaining_time(),
            self.resources.fonts.level_font,
        )?;

        let total_score_text = format!("Total Score: {}", self.level_score + self.total_score);
        let total_score_text_to_draw = DrawText::new(
            Point2::from_slice(&[10.0, 50.0]),
            total_score_text,
            30.0,
            Color::WHITE,
        )?;
        draw_score(
            ctx,
            total_score_text_to_draw,
            self.resources.fonts.score_font,
        )?;

        let lives_text_to_draw = DrawText::new(
            Point2::from_slice(&[530.0, 60.0]),
            format!("Lives: {}", self.lives),
            32.0,
            Color::WHITE,
        )?;
        draw_text(ctx, lives_text_to_draw, self.resources.fonts.lives_font)?;

        if self.is_paused {
            draw_icon(ctx, &self.pause_button)?;
        }

        Ok(())
    }

    fn update_next_level(&mut self, ctx: &mut Context) -> Result<(), DrawError> {
        if !self.level_complete_sound_played {
            self.audio.play_sound(ctx, "level_completed".to_string())?;
            self.level_complete_sound_played = true;
        }

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
            self.reset(ctx)?;
        }

        Ok(())
    }

    fn draw_next_level(&mut self, ctx: &mut Context) -> Result<(), DrawError> {
        draw_background(ctx, &self.resources.background_image)?;
        let level_complete_text = DrawText::new(
            Point2::from_slice(&[WINDOW_WIDTH / 2.0 - 160.0, WINDOW_HEIGHT / 2.0 - 100.0]),
            "Level Complete!".to_string(),
            48.0,
            Color::WHITE,
        )?;
        draw_text(ctx, level_complete_text, self.resources.fonts.level_font)?;
        draw_button_with_text(
            ctx,
            self.next_level_button.clone(),
            self.resources.fonts.level_font,
        )?;
        Ok(())
    }

    fn update_game_over(&mut self, ctx: &mut Context) -> Result<(), DrawError> {
        if !self.game_over_sound_played {
            self.audio.play_sound(ctx, "game_over".to_string())?;
            self.game_over_sound_played = true;
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
            self.current_level = 0;
            self.reset(ctx)?;
        }
        Ok(())
    }

    fn draw_game_over(&mut self, ctx: &mut Context) -> Result<(), DrawError> {
        draw_background(ctx, &self.resources.background_image)?;
        let game_over_text = DrawText::new(
            Point2::from_slice(&[WINDOW_WIDTH / 2.0 - 90.0, WINDOW_HEIGHT / 2.0 - 75.0]),
            "Game Over".to_string(),
            48.0,
            Color::WHITE,
        )?;
        draw_text(ctx, game_over_text, self.resources.fonts.score_font)?;

        draw_button_with_text(
            ctx,
            self.restart_button.clone(),
            self.resources.fonts.lives_font,
        )?;
        Ok(())
    }

    fn update_victory(&mut self, ctx: &mut Context) -> Result<(), DrawError> {
        if !self.victory_sound_played {
            self.audio.play_sound(ctx, "victory".to_string())?;
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
            self.current_level = 0;
            self.reset(ctx)?;
        }
        Ok(())
    }
    fn draw_victory(&mut self, ctx: &mut Context) -> Result<(), DrawError> {
        draw_background(ctx, &self.resources.background_image)?;
        let game_complete_text = DrawText::new(
            Point2::from_slice(&[WINDOW_WIDTH / 2.0 - 195.0, WINDOW_HEIGHT / 2.0 - 125.0]),
            "You Win! Game Over".to_string(),
            48.0,
            Color::from_rgb(0, 255, 100),
        )?;
        draw_text(ctx, game_complete_text, self.resources.fonts.level_font)?;

        let final_score_text = format!("Final Score: {}", self.total_score + self.level_score);
        let final_score_text_to_draw = DrawText::new(
            Point2::from_slice(&[WINDOW_WIDTH / 2.0 - 100.0, WINDOW_HEIGHT / 2.0 - 75.0]),
            final_score_text,
            32.0,
            Color::WHITE,
        )?;
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
        Ok(())
    }

    fn update_select_level(&mut self, ctx: &mut Context) -> Result<(), DrawError> {
        let levels = self.levels.clone();
        for (i, _) in levels.iter().enumerate() {
            let level_button = get_level_button(i)?;

            if is_button_clicked(
                ctx,
                Rect::new(
                    level_button.coords.x,
                    level_button.coords.y,
                    level_button.size.w,
                    level_button.size.h,
                ),
            ) {
                self.current_level = i;
                self.reset(ctx)?;
            }
        }

        Ok(())
    }

    fn draw_select_level(&mut self, ctx: &mut Context) -> Result<(), DrawError> {
        draw_background(ctx, &self.resources.menu_background_image)?;
        for (i, _) in self.levels.iter().enumerate() {
            let level_button = get_level_button(i)?;

            draw_button_with_text(ctx, level_button, self.resources.fonts.lives_font)?;
        }

        Ok(())
    }
}

impl EventHandler<GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.game_mode {
            GameMode::Menu => self.update_menu(ctx),
            GameMode::Playing => self.update_playing(ctx),
            GameMode::GameOver => self.update_game_over(ctx),
            GameMode::NextLevel => self.update_next_level(ctx),
            GameMode::Victory => self.update_victory(ctx),
            GameMode::LevelSelection => self.update_select_level(ctx),
        }?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, Color::from_rgb(0, 0, 0));
        draw_background(ctx, &self.resources.background_image)?;

        match self.game_mode {
            GameMode::Menu => self.draw_menu(ctx),
            GameMode::Playing => self.draw_playing(ctx),
            GameMode::GameOver => self.draw_game_over(ctx),
            GameMode::NextLevel => self.draw_next_level(ctx),
            GameMode::Victory => self.draw_victory(ctx),
            GameMode::LevelSelection => self.draw_select_level(ctx),
        }?;

        present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Space => match self.is_paused {
                true => self.resume(),
                false => self.pause(),
            },
            KeyCode::Left => {
                if self.player.coords.x > 0.0 {
                    self.player.move_left();
                }
            }
            KeyCode::Right => {
                if self.player.coords.x < WINDOW_WIDTH - self.player.size.w {
                    self.player.move_right();
                }
            }
            _ => (),
        }
    }
}
