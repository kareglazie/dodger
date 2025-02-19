#[derive(PartialEq, Debug)]
pub enum GameMode {
    Menu,
    LevelSelection,
    Playing,
    Paused,
    GameOver,
    Victory,
    NextLevel,
}
