use dodger::gamestate::GameState;
use dodger::resources::Resources;
use dodger::sound::AudioManager;
use ggez::event;
use ggez::{ContextBuilder, GameResult};

fn main() -> GameResult<()> {
    let (mut ctx, event_loop) = ContextBuilder::new("dodger", "author")
        .add_resource_path("./resources")
        .build()
        .expect("Could not create ggez context!");

    let audio_manager = AudioManager::new(&mut ctx);
    let resources = Resources::load_level(&mut ctx, 0);
    let state = GameState::new(&mut ctx, resources, 0, audio_manager)?;
    event::run(ctx, event_loop, state)
}
