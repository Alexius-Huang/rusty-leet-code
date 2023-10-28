/**
 *  Problem 206. Reversed Linked List (Easy)
 *  See: https://leetcode.com/problems/reverse-linked-list/
 *
 *  Given the head of a singly linked list, reverse the list, and
 *  return the reversed list.
 */

type WrappedListNode = Option<Box<ListNode>>;

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

        head.next  = Some(Box::new(next_node));
        head
    }
}

pub fn run_iteratively(mut head: WrappedListNode) -> WrappedListNode {
    let mut prev = None;

    while let Some(mut head_node) = head.take() {
        head = std::mem::replace(&mut head_node.next, prev);
        prev = Some(head_node);
    }

    prev    
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_reverses_linked_list() {
        let input = ListNode::build(
            vec![1, 2, 3, 4, 5]
        );

        let result = ListNode::build(
            vec![5, 4, 3, 2, 1]
        );

        assert_eq!(run_iteratively(Some(Box::new(input))), Some(Box::new(result)));
    }
}

