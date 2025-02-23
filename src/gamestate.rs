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

/// **Represents the state of the game, including all game objects, UI elements, and game logic.**
///
/// ## Fields
/// * `total_score`: the player's total score across all levels.
/// * `level_score`: the player's score for the current level.
/// * `current_level`: the index of the current level being played.
/// * `levels`: a list of all available levels.
/// * `resources`: the game resources, including images, fonts, and sounds.
/// * `player`: the player object.
/// * `falling_objects`: a list of objects currently falling in the game.
/// * `last_update`: the timestamp of the last game update.
/// * `level_start_time`: the timestamp when the current level started.
/// * `paused_time`: the timestamp when the game was paused, if applicable.
/// * `audio`: the audio manager for playing sounds.
/// * `audio_button`: the button to toggle audio on/off.
/// * `start_button`: the button to start the game.
/// * `exit_button`: the button to exit the game.
/// * `resume_button`: the button to resume the game from pause.
/// * `menu_button`: the button to enter the main menu.
/// * `back_to_menu_button`: the button to return to the main menu from the "How to Play" screen.
/// * `pause_button`: the button to pause the game.
/// * `next_level_button`: the button to proceed to the next level.
/// * `restart_button`: the button to restart the current level or the game.
/// * `select_level_button`: the button to open the level selection screen.
/// * `howtoplay_button`: the button to open the "How to Play" screen.
/// * `lives`: the number of lives the player has remaining.
/// * `game_mode`: the current mode of the game (e.g., Menu, Playing, GameOver).
/// * `level_complete_sound_played`: whether the level complete sound has been played.
/// * `victory_sound_played`: whether the victory sound has been played.
/// * `game_over_sound_played`: whether the game over sound has been played.
/// * `game_started`: whether the game has started.
/// * `is_paused`: whether the game is currently paused.
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
    back_to_menu_button: TextButton,
    pause_button: IconButton,
    next_level_button: TextButton,
    restart_button: TextButton,
    select_level_button: TextButton,
    howtoplay_button: TextButton,
    lives: u8,
    game_mode: GameMode,
    level_complete_sound_played: bool,
    victory_sound_played: bool,
    game_over_sound_played: bool,
    game_started: bool,
    is_paused: bool,
}

impl GameState {
    /// **Initializes a new `GameState` with default values and resources.**
    ///
    /// ## Parameters
    /// * `ctx`: the game context.
    /// * `resources`: the game resources (images, fonts, sounds).
    /// * `current_level`: index of the starting level.
    /// * `audio_manager`: the audio manager for playing sounds.
    ///
    /// ## Returns
    /// A result containing the initialized `GameState`, or a `DodgerError` if initialization fails.
    ///
    /// ## Behavior
    /// * Loads fonts.
    /// * Initializes the player, buttons, and other game objects.
    /// * Sets up the initial game state (e.g., score, lives, game mode).
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

        let howtoplay_button = TextButton::new(
            start_point_of_button_in_set(2, 300.0),
            Color::WHITE,
            default_text_button_size,
            "How to Play".to_string(),
            Color::BLACK,
            BUTTON_TEXT_SIZE,
            "button_font".to_string(),
        )?;

        let exit_button = TextButton::new(
            start_point_of_button_in_set(3, 300.0),
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

        let back_to_menu_button = TextButton::new(
            Point2::from_slice(&[WINDOW_WIDTH - 200.0, 10.0]),
            Color::WHITE,
            RectSize::from((100.0, 40.0)),
            "Back".to_string(),
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
            back_to_menu_button,
            pause_button,
            next_level_button,
            restart_button,
            select_level_button,
            howtoplay_button,
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

    /// **Creates a new falling object and adds it to the game.**
    ///
    /// ## Returns
    /// `Ok(())` if the object is created successfully, or a `DodgerError` if creation fails.
    ///
    /// ## Behavior
    /// * Randomly generates a horizontal position for the object.
    /// * Determines if the object is "good" or "bad".
    /// * Assigns a value to "good" objects (`High`, `Medium`, `Low`).
    /// * Adds the object to the `falling_objects` list.
    fn create_falling_object(&mut self) -> Result<(), DodgerError> {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(25.0..WINDOW_WIDTH - 25.0);
        let is_good = self.falling_objects.len() % 5 != 0;
        let good_object_value = if is_good {
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

    ///**Handles collisions between the player and falling objects.**
    ///
    /// ## Parameters
    /// `ctx`: the game context.
    ///
    /// ## Returns
    /// `Ok(())` if collisions are handled successfully, or a `DodgerError` if sound playback fails.
    ///
    /// ## Behavior
    /// * Checks for collisions between the player and each falling object.
    /// * Updates the score if the player catches a "good" object.
    /// * Reduces lives if the player collides with a "bad" object.
    /// * Plays appropriate sounds for collisions.
    /// * Removes objects that have been caught or have expired.
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
                    obj.remove_timer = Some(Instant::now());
                } else {
                    self.audio.play_sound(ctx, "bad_collision".to_string())?;
                    self.lives -= 1;
                    if self.lives == 0 {
                        self.game_mode = GameMode::GameOver;
                    }
                    obj.remove_timer = Some(Instant::now());
                    obj.blink_timer = Some(Instant::now());
                    self.player.blink_timer = Some(Instant::now());
                }
            }
        }

        self.falling_objects.retain(|obj| {
            if let Some(timer) = obj.remove_timer {
                !(obj.is_good || timer.elapsed() >= Duration::from_secs(1))
            } else {
                true
            }
        });
        Ok(())
    }

    /// **Resets the game state for a new level or restart.**
    ///
    /// ## Parameters
    /// `ctx`: the game context.
    ///
    /// ## Returns
    /// `Ok(())` if the reset is successful, or a `DodgerError` if resource loading fails.
    ///
    /// ## Behavior
    /// * Resets the score, lives, and timers.
    /// * Clears the list of falling objects.
    /// * Loads resources for the current level.
    /// * Sets the game mode to `Playing`.
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

    /// **Pauses the game.**
    ///
    /// ## Behavior
    /// * Records the current time as the pause start time.
    /// * Sets `is_paused` to `true`.
    fn pause(&mut self) {
        if self.game_mode == GameMode::Playing {
            self.paused_time = Some(Instant::now());
            self.is_paused = true;
        }
    }

    /// **Resumes the game from pause.**
    ///
    /// ## Behavior
    /// * Calculates the duration of the pause.
    /// * Adjusts the game timers to account for the pause duration.
    /// * Sets `is_paused` to `false`.
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

    /// **Calculates the remaining time for the current level.**
    ///
    /// ## Returns
    /// The remaining time in seconds.
    ///
    /// ## Behavior
    /// * If the game is paused, calculates the remaining time based on the pause start time.
    /// * If the game is not paused, calculates the remaining time based on the current time.
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

    /// **Updates the game state when in the main menu.**
    ///
    /// ## Parameters
    /// `ctx`: the game context.
    ///
    /// ## Returns
    /// `Ok(())` if the update is successful, or a 'DodgerError` if button handling fails.
    ///
    /// ## Behavior
    /// Handles button clicks for starting/resuming the game, selecting levels, opening the "How to Play" screen, and exiting the game.
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

        if is_button_clicked(ctx, text_button_rect(&self.howtoplay_button)?) {
            self.game_mode = GameMode::HowToPlay;
        }

        if is_button_clicked(ctx, text_button_rect(&self.exit_button)?) {
            ctx.request_quit();
        }
        Ok(())
    }

    /// **Draws the main menu on the canvas.**
    ///
    /// ## Parameters
    /// * `ctx`: the game context.
    /// * `canvas`: canvas to draw on.
    ///
    /// ## Returns
    /// `Ok(())` if drawing is successful, or a `DodgerError` if button drawing fails.
    fn draw_menu(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> Result<(), DodgerError> {
        draw_background(canvas, &self.resources.menu_background_image);
        if !self.game_started {
            draw_button_with_text(ctx, canvas, self.start_button.clone())?;
        } else {
            draw_button_with_text(ctx, canvas, self.resume_button.clone())?;
        }
        draw_button_with_text(ctx, canvas, self.exit_button.clone())?;
        draw_button_with_text(ctx, canvas, self.select_level_button.clone())?;
        draw_button_with_text(ctx, canvas, self.howtoplay_button.clone())?;
        Ok(())
    }

    /// **Updates the game state when in the playing mode.**
    ///
    /// ## Parameters
    /// `ctx`: the game context.
    ///
    /// ## Returns
    /// `Ok(())` if the update is successful, or a `DodgerError` if object creation or collision handling fails.
    ///
    /// ## Behavior
    /// * Handles button clicks for toggling audio and returning to the menu.
    /// * Updates falling objects and checks for collisions.
    /// * Advances to the next level or victory screen if the level is complete.
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

    /// **Draws the game state when in the playing mode.**
    ///
    /// ## Parameters
    /// * `ctx`: the game context.
    /// * `canvas`: canvas to draw on.
    ///
    /// ## Returns
    /// `Ok(())` if drawing is successful, or a `DodgerError` if text or button drawing fails.
    ///
    /// ## Behavior
    /// Draws the background, player, falling objects, and UI elements (score, timer, lives).
    fn draw_playing(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> Result<(), DodgerError> {
        draw_background(canvas, &self.resources.background_image);
        self.player.draw(canvas);
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
            obj.draw(canvas);
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

    /// **Updates the game state when in the "Next Level" mode.**
    ///
    /// ## Parameters
    /// `ctx`: the game context.
    ///
    /// ## Returns
    /// `Ok(())` if the update is successful, or a `DodgerError` if sound playback fails.
    ///
    /// ## Behavior
    /// * Plays the "level completed" sound.
    /// * Handles button clicks for proceeding to the next level.
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

    /// **Draws the "Next Level" screen on the canvas.**
    ///
    /// ## Parameters
    /// * `ctx`: the game context.
    /// * `canvas`: canvas to draw on.
    ///
    /// ## Returns
    /// `Ok(())` if drawing is successful, or a `DodgerError` if text or button drawing fails.
    ///
    /// ## Behavior
    /// Draws the background, "Level Complete" text, and a button to proceed to the next level.
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
    /// **Updates the game state when in "How to Play" screen.**
    ///
    /// ## Parameters
    /// `ctx`: the game context.
    ///
    /// ## Returns
    /// `Ok(())` if the update is successful, or a `DodgerError` if button handling fails.
    fn update_how_to_play(&mut self, ctx: &mut Context) -> Result<(), DodgerError> {
        if is_button_clicked(ctx, text_button_rect(&self.back_to_menu_button)?) {
            self.game_mode = GameMode::Menu;
        }
        Ok(())
    }

    /// **Draws the "How to Play" screen on the canvas.**
    ///
    /// ## Parameters
    /// * `ctx`: the game context.
    /// * `canvas`: canvas to draw on.
    ///
    /// ## Returns
    /// `Ok(())` if drawing is successful, or a `DodgerError` if text or button drawing fails.
    ///
    /// ## Behavior
    /// Draws the background, title, instructions, and a button to return to the main menu.
    fn draw_how_to_play(
        &mut self,
        ctx: &mut Context,
        canvas: &mut Canvas,
    ) -> Result<(), DodgerError> {
        draw_background(canvas, &self.resources.menu_background_image);

        let title = DrawText::new(
            Point2::from_slice(&[WINDOW_WIDTH / 2.0 - 150.0, 150.0]),
            "How to Play".to_string(),
            "text_font".to_string(),
            48.0,
            Color::WHITE,
        )?;
        draw_text(canvas, title)?;

        let instructions = vec![
            "Use Left/Right arrows to move the player.",
            "Press Space to pause the game.",
            "Catch good objects to earn points:",
            "  - High value: 30 points",
            "  - Medium value: 15 points",
            "  - Low value: 5 points",
            "Avoid bad objects! They reduce your lives.",
            "Each level lasts 40 seconds.",
        ];

        let mut y_offset = 250.0;
        for line in instructions {
            let instruction_text = DrawText::new(
                Point2::from_slice(&[150.0, y_offset]),
                line.to_string(),
                "text_font".to_string(),
                TEXT_SIZE,
                Color::WHITE,
            )?;
            draw_text(canvas, instruction_text)?;
            y_offset += 50.0;
        }

        draw_button_with_text(ctx, canvas, self.back_to_menu_button.clone())?;

        Ok(())
    }
    /// **Updates the game state when in "Game Over" mode.**
    ///
    /// ## Parameters
    /// `ctx`: the game context.
    ///
    /// ## Returns
    /// `Ok(())` if the update is successful, or a `DodgerError` if sound playback fails.
    ///
    /// ## Behavior
    /// * Plays the "game over" sound.
    /// * Handles button clicks for restarting the game.
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
    /// **Draws the "Game Over" screen on the canvas.**
    ///
    /// ## Parameters
    /// * `ctx`: the game context.
    /// * `canvas`: canvas to draw on.
    ///
    /// ## Returns
    /// `Ok(())` if drawing is successful, or a `DodgerError` if text or button drawing fails.
    ///
    /// ## Behavior
    /// Draws the background, "Game Over" text, and a restart button.
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

    /// **Updates the game state when in "Victory" mode.**
    ///
    /// ## Parameters
    /// `ctx`: the game context.
    ///
    /// ## Returns
    /// `Ok(())` if the update is successful, or a `DodgerError` error if sound playback fails.
    ///
    /// ## Behavior
    /// * Plays the "victory" sound.
    /// * Handles button clicks for restarting the game.
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

    /// **Draws the "Victory" screen on the canvas.**
    ///
    /// ## Parameters
    /// * `ctx`: the game context.
    /// * `canvas`: ccanvas to draw on.
    ///
    /// ## Returns
    /// `Ok(())` if drawing is successful, or a `DodgerError` if text or button drawing fails.
    ///
    /// ## Behavior
    /// Draws the background, victory text, final score, and a restart button.
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

    /// **Updates the game state when in "Level Selection" mode.**
    ///
    /// ## Parameters
    /// `ctx`: the game context.
    ///
    /// ## Returns
    /// `Ok(())` if the update is successful, or a `DodgerError` if button handling fails.
    ///
    /// ## Behavior
    /// * Handles button clicks for selecting a level.
    /// * Resets the game state to start the selected level.
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

    /// **Draws the "Level Selection" screen on the canvas.**
    ///
    /// ## Parameters
    /// * `ctx`: the game context.
    /// * `canvas`: canvas to draw on.
    ///
    /// ## Returns
    /// `Ok(())` if drawing is successful, or a `DodgerError` if button drawing fails.
    ///
    /// ## Behavior
    /// Draws the background and buttons for each available level.
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
            GameMode::HowToPlay => self.update_how_to_play(ctx),
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
            GameMode::HowToPlay => self.draw_how_to_play(ctx, &mut canvas),
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
