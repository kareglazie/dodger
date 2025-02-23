#[derive(PartialEq, Debug)]
pub enum GameMode {
    Menu,
    LevelSelection,
    Playing,
    HowToPlay,
    NextLevel,
    GameOver,
    Victory,
}
