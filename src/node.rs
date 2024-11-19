pub struct ListNode {
    val: i32,
    next_node: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(&mut self, val: i32) {
        ListNode {
            val,
            next_node: None,
        };
        self.set_val(val);
    }

    pub fn set_val(&mut self, val: i32) {
        self.val = val;
    }

    pub fn get_val(self) -> i32 {
        self.val
    }
}
