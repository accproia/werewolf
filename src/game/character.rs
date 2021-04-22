use log::{error, info};
use std::option::Option;

pub struct Character {
    _telegram_id: Option<String>, 
    _name: String,
}

impl Character {

    pub fn new(name: &str) -> Character {
        Character{
            _telegram_id: None,
            _name: String::from(name),
        }
    }

    pub fn name(&self) -> &str {
        &self._name[..]
    }

    pub fn telegram_id(&self) -> &Option<String> {
        &self._telegram_id
    }

    pub fn is_real_user(&self) -> bool {
        match &self._telegram_id {
            Some(_) => true,
            _ => false
        } 
    }

    pub fn load_user(&mut self, telegram_id: &str) {
        if let Some(current_id) = &self._telegram_id {
            error!("Character is already owned by {}", current_id);
            return;
        }        

        self._telegram_id = Some(String::from(telegram_id));
        info!("Character {} is now owned by {}", &self._name, &telegram_id);
    }

    pub fn unload_user(&mut self) {
        if !self.is_real_user() {
            error!("Character {} isn't owned by anyone", &self._name);
            return;
        }

        self._telegram_id = None;
    }
}
