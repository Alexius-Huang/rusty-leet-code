pub type WrappedListNode = Option<Box<ListNode>>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: WrappedListNode,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn wrap(val: i32) -> Option<Box<Self>> {
        Some(Box::new(Self::new(val)))
    }

    pub fn log(&self) {
        let mut result = vec![self.val];

        let mut current_node = &self.next;
        while let Some(node) = current_node.as_ref() {
            result.push(node.val);
            current_node = &node.next;
        }

        println!("LinkedList {result:?}");
    }

    pub fn build(values: Vec<i32>) -> Self {
        if values.len() == 0 {
            panic!("Cannot build linked list with empty values");
        }

        let mut head = Self::new(values[0]);

        if values.len() == 1 {
            return head;
        }

        let len = values.len();
        let mut next_node = ListNode::new(values[len - 1]);

        for i in (1..(values.len() - 1)).rev() {
            let mut prev_node = ListNode::new(values[i]);
            prev_node.next = Some(Box::new(next_node));
            next_node = prev_node;
        }

        head.next = Some(Box::new(next_node));
        head
    }
}
