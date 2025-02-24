#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dodger::consts::{WINDOW_HEIGHT, WINDOW_WIDTH};
use dodger::gamestate::GameState;
use dodger::levels::get_levels;
use dodger::resources::Resources;
use dodger::sound::AudioManager;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::{ContextBuilder, GameResult};

fn main() -> GameResult<()> {
    let window_mode = WindowMode {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        resizable: false,
        ..WindowMode::default()
    };
    let (mut ctx, event_loop) = ContextBuilder::new("dodger", "me")
        .add_resource_path("./resources")
        .window_setup(WindowSetup::default().title("My Awesome Game"))
        .window_mode(window_mode)
        .build()?;

    let audio_manager = AudioManager::new(&mut ctx)?;
    let levels = get_levels();
    let resources = Resources::load_level(&mut ctx, 0, &levels)?;
    let state = GameState::new(&mut ctx, resources, 0, audio_manager)?;
    event::run(ctx, event_loop, state)
}
