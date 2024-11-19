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
        let node_aux = Box::new(node::ListNode::new(val));
        
        if self.is_empty() {
            self.head = Some(node_aux);
            return true;
        } else {
            let mut current = &mut self.head;
            
            while let Some(ref mut node) = current {
                if node.next_node.is_none() {
                    node.next_node = Some(node_aux);
                    return true;
                }
                current = &mut node.next_node;
            }
        }
        return false;
    }
}
