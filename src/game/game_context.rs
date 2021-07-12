use std::any::Any;
use std::collections::HashMap;
use std::boxed::Box;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, Arc, Weak, Mutex};
use super::user_context::UserContext;

type DataType = HashMap<String, Box<dyn Any>>;


pub struct GameContext {
    _data: RwLock<DataType>,
    _users: RwLock<Vec<Weak<UserContext>>>,
    _aboutToWrite: Mutex<u32>,
}

impl GameContext {

    pub fn new() -> GameContext {
        GameContext{
            _data: RwLock::new(DataType::default()),
            _users: RwLock::new(Vec::default()),
            _aboutToWrite: Mutex::new(0)
        }
    }

    fn acquire_reader(&self) -> RwLockReadGuard<Vec<Weak<UserContext>>>{
        let guard = match self._aboutToWrite.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner()
        };
        match self._users.read() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    fn acquire_writer(&self, read_guard: RwLockReadGuard<Vec<Weak<UserContext>>>) -> RwLockWriteGuard<Vec<Weak<UserContext>>>{
        match self._users.write() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    pub fn has_user(&self, user: Arc<UserContext>) -> bool {
        let usersReader = self.acquire_reader();
        let userWeak = Arc::downgrade(&user);
        let userOpt = usersReader.into_iter().find(|&x| x.ptr_eq(&userWeak));
        match userOpt {
            Some(_) => true,
            _ => false
        }
    }

    pub fn add_user(&mut self, user: Arc<UserContext>) -> bool {

        let mut usersReader = match self._users.read() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let userWeak = Arc::downgrade(&user);
        let userOpt = usersReader.into_iter().find(|&x| x.ptr_eq(&userWeak));
        if let Some(_) = userOpt {
            return false;
        }

        let usersWriter = self.acquire_writer(usersReader);
        usersWriter.push(userWeak);
        true
    }

    pub fn remove_user_base<P>(&mut self, predicate: P) -> bool
    where P: Fn(&mut RwLockReadGuard<Vec<Weak<UserContext>>>) -> Option<usize> {

        let mut usersReader = match self._users.read() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let idxOpt = predicate(&mut usersReader);
        if let None = idxOpt {
            return false;
        }
        let idx = idxOpt.unwrap();
        let usersWriter = self.acquire_writer(usersReader);
        usersWriter.remove(idx);
        true
    }
    
    pub fn remove_user(&mut self, user: Arc<UserContext>) -> bool {
        self.remove_user_base( |reader: &mut RwLockReadGuard<Vec<Weak<UserContext>>>| {
            let userWeak = Arc::downgrade(&user);
            reader.iter().position(|&x| x.ptr_eq(&userWeak)) 
        })
    }

    pub fn remove_user_by_tgid(&mut self, telegram_id: &str) -> bool {
        self.remove_user_base( |reader: &mut RwLockReadGuard<Vec<Weak<UserContext>>>| {
            reader.iter().position(|&x| x.unwrap().telegram_id == telegram_id) 
        })
    }
}
