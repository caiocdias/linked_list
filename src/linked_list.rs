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

    pub fn remove_node_by_index(&mut self, index: i32) -> (bool, &str) {
        if self.is_empty() {
            return(false, "List is empty");
        } 

        else if index <= 0 || index > self.lenght {
            return (false, "Index out of array");
        } 

        else if index == 1 {
            self.head = self.head.next_node;
            return (true, "Node successfully removed from head of the list");
        }

        else {
            let current = &mut self.head;
            let next = &mut self.head.next_node;
            let count: i32 = 0;
            while let Some(ref mut node) = current {
                if count == index - 1 {
                    current.next_node = &mut next.next_node;
                    return (true, "Node successfully removed from the list");
                } else {
                    next = &mut next.next_node;
                    current = &mut current.next_node
                }
            }
            return (false, "Failed to remove the node of the list");
        }
    }
}
