use std::sync::{Weak, Arc};
use log::error;
use super::game_context::GameContext;


pub struct UserContext {
    pub _telegram_id: String,
    pub _name: Option<String>,
    pub _surname: Option<String>,
    pub _game: Weak<GameContext>,
}


impl UserContext {
    pub fn new( telegram_id: &str, name: Option<&str>, surname: Option<&str> ) -> UserContext {
        let mut uc = UserContext {
            _telegram_id: String::from(telegram_id),
            _name: None,
            _surname: None,
            _game: Weak::default()
        };

        if let Some(name) = name {
            uc._name = Some(String::from(name));
        }

        if let Some(surname) = surname {
            uc._surname = Some(String::from(surname));
        }
        
        uc
    }

    pub fn start_game(&mut self, game: Arc<GameContext>)
    {
        let oldGameRef = self._game.upgrade();
        if let Some(oldGameRef) = oldGameRef {
            error!(
                "User(id: {}{}{}) is already in the game. Trying to end this game and start new one.",
                &self._telegram_id, 
                if let Some(name) = &self._name {String::from(", name: ") + name} else {String::new()},
                if let Some(surname) = &self._surname {String::from(", surname: ") + surname} else {String::new()}
            ); 
            oldGameRef.remove_user(self);
        }
        self._game = Arc::downgrade(&game);
    }  
}