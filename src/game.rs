pub enum GameState {
    GAME_ACTIVE,
    GAME_MENU,
    GAME_WIN
}

pub struct Game {
    pub state: GameState,
    pub keys: Vec<bool>,
    pub width: u32,
    pub height: u32,
}