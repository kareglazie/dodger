use std::time::{Duration, Instant};

use ggez::{
    event::EventHandler,
    graphics::{Canvas, Color},
    input::keyboard::{KeyCode, KeyInput},
    mint::{Point2, Vector2},
    Context, GameError, GameResult,
};

use rand::Rng;

use crate::{
    buttons::{DrawText, IconButton, TextButton},
    consts::{
        BUTTON_TEXT_SIZE, FALLING_OBJECT_UPDATE_MILLIS, LEVEL_DURATION_SECS, LIVES, TEXT_SIZE,
        WINDOW_HEIGHT, WINDOW_WIDTH,
    },
    errors::DodgerError,
    levels::{get_levels, Level},
    modes::GameMode,
    objects::{FallingObject, GoodObjectValue},
    player::Player,
    resources::{add_fonts, Resources},
    sound::AudioManager,
    ui::{draw_background, draw_button_with_text, draw_icon, draw_score, draw_text, draw_timer},
    utils::{
        get_level_button, half_scaling, icon_button_rect, is_button_clicked, object_scaling,
        player_scaling, start_point_of_button_in_set, start_point_of_centered_button,
        text_button_rect, text_button_rectsize, RectSize,
    },
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
    ) -> Result<Self, DodgerError> {
        add_fonts(ctx)?;
        let player = Player::new(
            ctx,
            Point2::from_slice(&[WINDOW_WIDTH / 2.0, WINDOW_HEIGHT - 175.0]),
            player_scaling(),
            &resources.player_image,
        )?;
        let default_text_button_size = text_button_rectsize();
        let restart_button = TextButton::new(
            start_point_of_centered_button(),
            Color::WHITE,
            default_text_button_size,
            "Restart".to_string(),
            Color::BLACK,
            BUTTON_TEXT_SIZE,
            "button_font".to_string(),
        )?;

        let audio_button = IconButton::new(
            Point2::from_slice(&[WINDOW_WIDTH - 85.0, 60.0]),
            Vector2::from_slice(&[0.15, 0.15]),
            audio_manager.speaker_icon,
        )?;

        let pause_button = IconButton::new(
            Point2::from_slice(&[WINDOW_WIDTH / 2.0 - 40.0, WINDOW_HEIGHT / 2.0 - 30.0]),
            half_scaling(),
            resources.pause_button_image.clone(),
        )?;

        let next_level_button = TextButton::new(
            start_point_of_centered_button(),
            Color::WHITE,
            default_text_button_size,
            "Next Level".to_string(),
            Color::BLACK,
            BUTTON_TEXT_SIZE,
            "button_font".to_string(),
        )?;

        let start_button = TextButton::new(
            start_point_of_button_in_set(0, 300.0),
            Color::WHITE,
            default_text_button_size,
            "Start".to_string(),
            Color::BLACK,
            BUTTON_TEXT_SIZE,
            "button_font".to_string(),
        )?;

        let resume_button = TextButton::new(
            start_point_of_button_in_set(0, 300.0),
            Color::WHITE,
            default_text_button_size,
            "Resume".to_string(),
            Color::BLACK,
            BUTTON_TEXT_SIZE,
            "button_font".to_string(),
        )?;

        let select_level_button = TextButton::new(
            start_point_of_button_in_set(1, 300.0),
            Color::WHITE,
            default_text_button_size,
            "Select Level".to_string(),
            Color::BLACK,
            BUTTON_TEXT_SIZE,
            "button_font".to_string(),
        )?;

        let exit_button = TextButton::new(
            start_point_of_button_in_set(2, 300.0),
            Color::WHITE,
            default_text_button_size,
            "Exit".to_string(),
            Color::BLACK,
            BUTTON_TEXT_SIZE,
            "button_font".to_string(),
        )?;

        let menu_button = TextButton::new(
            Point2::from_slice(&[WINDOW_WIDTH - 200.0, 10.0]),
            Color::WHITE,
            RectSize::from((100.0, 40.0)),
            "Menu".to_string(),
            Color::BLACK,
            BUTTON_TEXT_SIZE,
            "button_font".to_string(),
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
            level_start_time: Instant::now(),
            paused_time: None,
            audio,
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

    fn create_falling_object(&mut self) -> Result<(), DodgerError> {
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
            object_scaling(),
            is_good,
            good_object_value,
            &self.resources,
        )?;

        self.falling_objects.push(object);

        Ok(())
    }

    fn handle_collisions(&mut self, ctx: &mut Context) -> Result<(), DodgerError> {
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

    fn reset(&mut self, ctx: &mut Context) -> Result<(), DodgerError> {
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
            player_scaling(),
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

    fn update_menu(&mut self, ctx: &mut Context) -> Result<(), DodgerError> {
        if !self.game_started {
            if is_button_clicked(ctx, text_button_rect(&self.start_button)?) {
                self.level_start_time = Instant::now();
                self.last_update = Instant::now();
                self.game_mode = GameMode::Playing;
            }
        } else if is_button_clicked(ctx, text_button_rect(&self.resume_button)?) {
            if let Some(paused_time) = self.paused_time {
                let pause_duration = paused_time.elapsed();
                self.last_update += pause_duration;
                self.level_start_time += pause_duration;
            }
            self.paused_time = None;
            self.game_mode = GameMode::Playing;
        }

        if is_button_clicked(ctx, text_button_rect(&self.select_level_button)?) {
            self.game_mode = GameMode::LevelSelection;
        }

        if is_button_clicked(ctx, text_button_rect(&self.exit_button)?) {
            ctx.request_quit();
        }
        Ok(())
    }

    fn draw_menu(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> Result<(), DodgerError> {
        draw_background(canvas, &self.resources.menu_background_image);
        if !self.game_started {
            draw_button_with_text(ctx, canvas, self.start_button.clone())?;
        } else {
            draw_button_with_text(ctx, canvas, self.resume_button.clone())?;
        }
        draw_button_with_text(ctx, canvas, self.exit_button.clone())?;
        draw_button_with_text(ctx, canvas, self.select_level_button.clone())?;
        Ok(())
    }

    fn update_playing(&mut self, ctx: &mut Context) -> Result<(), DodgerError> {
        if !self.game_started {
            self.game_started = true;
        }

        if is_button_clicked(ctx, icon_button_rect(&self.audio_button)?) {
            self.audio.is_muted = !self.audio.is_muted;
        }

        if is_button_clicked(ctx, text_button_rect(&self.menu_button)?) {
            self.game_mode = GameMode::Menu;
            self.paused_time = Some(Instant::now());
        }

        if self.is_paused {
            return Ok(());
        }

        if self.level_start_time.elapsed() >= Duration::from_secs(LEVEL_DURATION_SECS) {
            if self.current_level + 1 < self.levels.len() {
                self.game_mode = GameMode::NextLevel;
            } else {
                self.game_mode = GameMode::Victory;
            }
        }

        if self.last_update.elapsed() >= Duration::from_millis(FALLING_OBJECT_UPDATE_MILLIS) {
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

    fn draw_playing(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> Result<(), DodgerError> {
        draw_background(canvas, &self.resources.background_image);
        self.player.draw(canvas)?;
        draw_button_with_text(ctx, canvas, self.menu_button.clone())?;

        let text = format!("Level {}", self.current_level + 1);
        let text_to_draw = DrawText::new(
            Point2::from_slice(&[525.0, 10.0]),
            text,
            "text_font".to_string(),
            TEXT_SIZE,
            Color::WHITE,
        )?;

        draw_text(canvas, text_to_draw)?;
        if self.audio.is_muted {
            self.audio_button.icon = self.audio.speaker_muted_icon.clone();
        } else {
            self.audio_button.icon = self.audio.speaker_icon.clone();
        };
        draw_icon(canvas, &self.audio_button)?;
        for obj in &mut self.falling_objects {
            obj.draw(canvas)?;
        }

        let level_score_text = format!("Level Score: {}", self.level_score);
        let level_score_text_to_draw = DrawText::new(
            Point2::from_slice(&[10.0, 10.0]),
            level_score_text,
            "text_font".to_string(),
            TEXT_SIZE,
            Color::WHITE,
        )?;
        draw_score(canvas, level_score_text_to_draw)?;

        draw_timer(ctx, canvas, self.get_remaining_time())?;

        let total_score_text = format!("Total Score: {}", self.level_score + self.total_score);
        let total_score_text_to_draw = DrawText::new(
            Point2::from_slice(&[10.0, 50.0]),
            total_score_text,
            "text_font".to_string(),
            TEXT_SIZE,
            Color::WHITE,
        )?;
        draw_score(canvas, total_score_text_to_draw)?;

        let lives_text_to_draw = DrawText::new(
            Point2::from_slice(&[530.0, 60.0]),
            format!("Lives: {}", self.lives),
            "text_font".to_string(),
            TEXT_SIZE,
            Color::WHITE,
        )?;
        draw_text(canvas, lives_text_to_draw)?;

        if self.is_paused {
            draw_icon(canvas, &self.pause_button)?;
        }

        Ok(())
    }

    fn update_next_level(&mut self, ctx: &mut Context) -> Result<(), DodgerError> {
        if !self.level_complete_sound_played {
            self.audio.play_sound(ctx, "level_completed".to_string())?;
            self.level_complete_sound_played = true;
        }

        if is_button_clicked(ctx, text_button_rect(&self.next_level_button)?) {
            self.current_level += 1;
            self.reset(ctx)?;
        }

        Ok(())
    }

    fn draw_next_level(
        &mut self,
        ctx: &mut Context,
        canvas: &mut Canvas,
    ) -> Result<(), DodgerError> {
        draw_background(canvas, &self.resources.background_image);
        let level_complete_text = DrawText::new(
            Point2::from_slice(&[WINDOW_WIDTH / 2.0 - 160.0, WINDOW_HEIGHT / 2.0 - 100.0]),
            "Level Complete!".to_string(),
            "text_font".to_string(),
            48.0,
            Color::WHITE,
        )?;
        draw_text(canvas, level_complete_text)?;
        draw_button_with_text(ctx, canvas, self.next_level_button.clone())?;
        Ok(())
    }

    fn update_game_over(&mut self, ctx: &mut Context) -> Result<(), DodgerError> {
        if !self.game_over_sound_played {
            self.audio.play_sound(ctx, "game_over".to_string())?;
            self.game_over_sound_played = true;
        }
        if is_button_clicked(ctx, text_button_rect(&self.restart_button)?) {
            self.current_level = 0;
            self.reset(ctx)?;
        }
        Ok(())
    }

    fn draw_game_over(
        &mut self,
        ctx: &mut Context,
        canvas: &mut Canvas,
    ) -> Result<(), DodgerError> {
        draw_background(canvas, &self.resources.background_image);
        let game_over_text = DrawText::new(
            Point2::from_slice(&[WINDOW_WIDTH / 2.0 - 95.0, WINDOW_HEIGHT / 2.0 - 75.0]),
            "Game Over".to_string(),
            "text_font".to_string(),
            48.0,
            Color::WHITE,
        )?;
        draw_text(canvas, game_over_text)?;

        draw_button_with_text(ctx, canvas, self.restart_button.clone())?;
        Ok(())
    }

    fn update_victory(&mut self, ctx: &mut Context) -> Result<(), DodgerError> {
        if !self.victory_sound_played {
            self.audio.play_sound(ctx, "victory".to_string())?;
            self.victory_sound_played = true;
        }
        if is_button_clicked(ctx, text_button_rect(&self.restart_button)?) {
            self.current_level = 0;
            self.reset(ctx)?;
        }
        Ok(())
    }

    fn draw_victory(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> Result<(), DodgerError> {
        draw_background(canvas, &self.resources.background_image);
        let game_complete_text = DrawText::new(
            Point2::from_slice(&[WINDOW_WIDTH / 2.0 - 185.0, WINDOW_HEIGHT / 2.0 - 125.0]),
            "You Win! Game Over".to_string(),
            "text_font".to_string(),
            48.0,
            Color::WHITE,
        )?;
        draw_text(canvas, game_complete_text)?;

        let final_score_text = format!("Final Score: {}", self.total_score + self.level_score);
        let final_score_text_to_draw = DrawText::new(
            Point2::from_slice(&[WINDOW_WIDTH / 2.0 - 110.0, WINDOW_HEIGHT / 2.0 - 75.0]),
            final_score_text,
            "text_font".to_string(),
            TEXT_SIZE,
            Color::WHITE,
        )?;
        draw_text(canvas, final_score_text_to_draw)?;
        draw_button_with_text(ctx, canvas, self.restart_button.clone())?;
        Ok(())
    }

    fn update_select_level(&mut self, ctx: &mut Context) -> Result<(), DodgerError> {
        let levels = self.levels.clone();
        for (i, _) in levels.iter().enumerate() {
            let level_button = get_level_button(i, 100.0, "button_font".to_string())?;

            if is_button_clicked(ctx, text_button_rect(&level_button)?) {
                self.current_level = i;
                self.reset(ctx)?;
            }
        }

        Ok(())
    }

    fn draw_select_level(
        &mut self,
        ctx: &mut Context,
        canvas: &mut Canvas,
    ) -> Result<(), DodgerError> {
        draw_background(canvas, &self.resources.menu_background_image);
        for (i, _) in self.levels.iter().enumerate() {
            let level_button = get_level_button(i, 100.0, "button_font".to_string())?;

            draw_button_with_text(ctx, canvas, level_button)?;
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
        let mut canvas = Canvas::from_frame(ctx, Color::from([0.0, 0.0, 0.0, 0.0]));

        match self.game_mode {
            GameMode::Menu => self.draw_menu(ctx, &mut canvas),
            GameMode::Playing => self.draw_playing(ctx, &mut canvas),
            GameMode::GameOver => self.draw_game_over(ctx, &mut canvas),
            GameMode::NextLevel => self.draw_next_level(ctx, &mut canvas),
            GameMode::Victory => self.draw_victory(ctx, &mut canvas),
            GameMode::LevelSelection => self.draw_select_level(ctx, &mut canvas),
        }?;

        canvas.finish(&mut ctx.gfx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: KeyInput,
        _repeat: bool,
    ) -> GameResult<()> {
        match input.keycode {
            Some(KeyCode::Space) => match self.is_paused {
                true => self.resume(),
                false => self.pause(),
            },
            Some(KeyCode::Left) => {
                if self.player.coords.x > 0.0 {
                    self.player.move_left();
                }
            }
            Some(KeyCode::Right) => {
                if self.player.coords.x < WINDOW_WIDTH - self.player.size.w {
                    self.player.move_right();
                }
            }
            _ => (),
        }
        Ok(())
    }
}
