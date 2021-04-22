use std::boxed::Box;

pub trait MenuElement {

    fn get_name(&self) -> String;
}

pub struct MenuStack {
    pub stack: Vec<Box<dyn MenuElement>>,
}

impl MenuStack {

    fn push(&mut self, elem:Box<dyn MenuElement>) {
        self.stack.push(elem);
    }

    fn pop(&mut self) {
        self.stack.pop();
    }
}