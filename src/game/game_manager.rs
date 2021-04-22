use super::game_context::GameContext;

pub struct GameManager {
    _games: Vec<Box<GameContext>>,
}

impl GameManager {

    pub fn new() -> GameManager {
        GameManager{_games: Vec::default()}
    }
}