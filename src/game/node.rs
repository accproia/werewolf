use std::rc::Weak;
use std::sync::Arc;

pub struct Node {
    name: String,
    parent: Weak<Node>,
    children: Vec<Arc<Node>>
}

impl Node {

    pub fn new(name: &str) -> Node {
        Node{
            name: String::from_str(name), 
            parent: Weak::<Node>::new(),
            children: Vec::<Arc<Node>>::new()
        }
    }
}