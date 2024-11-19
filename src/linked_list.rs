use crate::node;

pub struct LinkedList {
    head: Option<Box<node::ListNode>>,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList {
            head: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn insert_node(&mut self, val: i32) -> bool {
        true
    }

}
