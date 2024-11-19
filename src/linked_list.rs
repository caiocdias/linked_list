use crate::node;

pub struct LinkedList {
    head: Option<Box<node::ListNode>>,
    lenght: i32,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            lenght: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn insert_node(&mut self, val: i32) -> (bool, &str) {
        let node_aux = Box::new(node::ListNode::new(val));

        if self.is_empty() {
            self.head = Some(node_aux);
            self.lenght = self.lenght + 1;
            return (true, "Node successfully added as the head of the list");
        } else {
            let mut current = &mut self.head;

            while let Some(ref mut node) = current {
                if node.next_node.is_none() {
                    node.next_node = Some(node_aux);
                    self.lenght = self.lenght + 1;
                    return (true, "Node successfully added to the end of the list");
                }
                current = &mut node.next_node;
            }
        }
        return (false, "Failed to add the node to the list");
    }
    
    pub fn remove_node_by_index(&mut self, index: i32) -> (bool, &'static str) {
        if self.is_empty() {
            return (false, "List is empty");
        }

        if index <= 0 || index > self.lenght {
            return (false, "Index out of bounds");
        }

        if index == 1 {
            self.head = self.head.as_mut().and_then(|node| node.next_node.take());
            self.lenght -= 1;
            return (true, "Node successfully removed from the head of the list");
        }

        let mut current = &mut self.head;
        for _ in 1..index - 1 {
            if let Some(ref mut node) = current {
                current = &mut node.next_node;
            } else {
                return (false, "Failed to traverse the list");
            }
        }

        if let Some(ref mut node) = current {
            if let Some(ref mut next_node) = node.next_node {
                node.next_node = next_node.next_node.take();
                self.lenght -= 1;
                return (true, "Node successfully removed from the list");
            }
        }

        (false, "Failed to remove the node from the list")
    }
}
