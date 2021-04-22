use super::main_loop::MainLoop;
use super::game_manager::GameManager;
use std::boxed::Box;


pub struct AppContext {
    pub main_loop: Box<MainLoop>,
    pub games: Box<GameManager>,
}


impl AppContext {

    pub fn new(token: &str) -> AppContext {
        let main_loop = Box::<MainLoop>::new(MainLoop::new(token));
        AppContext{main_loop: main_loop, games: Box::new(GameManager::new())}
    }
}
