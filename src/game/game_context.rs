use std::any::Any;
use std::collections::HashMap;
use std::ops::Index;
use std::boxed::Box;
use std::sync::atomic::{Ordering, AtomicBool};
use log::error;
use super::character::Character;

//////////////////////////////////////////////
/// 
///     Helper class for lock/unlock
/// 
//////////////////////////////////////////////

pub struct LockGuard<'a> {
    _obj: &'a mut GameContext, 
}

impl<'a> Drop for LockGuard<'a> {
    fn drop(&mut self) {
        self._obj.unlock();
    }
}

//////////////////////////////////////////////
/// 
///             GameContext
/// 
//////////////////////////////////////////////

pub struct GameContext {
    _data: HashMap<String, Box<dyn Any>>,
    _users: Vec<Box<Character>>,
    _locked: AtomicBool,
}

impl GameContext {

    pub fn new() -> GameContext {
        GameContext{
            _data: HashMap::default(),
            _users: Vec::default(),
            _locked: AtomicBool::from(false)}
    }

    pub fn lock(&mut self) -> Result<LockGuard, ()> {
        if let Err(_) = self._locked.compare_exchange(
            false, 
            true, 
            Ordering::Acquire, 
            Ordering::Relaxed) {
            error!("GameContext data has already been locked");
            return Err(());
        }
        Ok(LockGuard{_obj: self})
    }

    fn unlock(&mut self) {
        if let Err(_) = self._locked.compare_exchange(
            true, 
            false, 
            Ordering::Acquire, 
            Ordering::Relaxed) {
            error!("GameContext data was unlocked twice.");
        }
    }
}

impl Index<&str> for GameContext {
    type Output = dyn Any;
    fn index(&self, id:&str) -> &Self::Output {
        &self._data[id]
    }
}