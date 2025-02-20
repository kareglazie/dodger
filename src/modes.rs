#[derive(PartialEq, Debug)]
pub enum GameMode {
    Menu,
    LevelSelection,
    Playing,
    NextLevel,
    GameOver,
    Victory,
}
