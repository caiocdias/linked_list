pub struct ListNode {
    val: i32,
    next_node: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        let mut node = ListNode {
            val: 0,
            next_node: None,
        };
        node.set_val(val);
        node
    }

    pub fn set_val(&mut self, val: i32) {
        self.val = val;
    }

    pub fn get_val(&self) -> i32 {
        self.val
    }
}
