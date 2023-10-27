/**
 *  Problem 206. Reversed Linked List (Easy)
 *  See: https://leetcode.com/problems/reverse-linked-list/
 *
 *  Given the head of a singly linked list, reverse the list, and
 *  return the reversed list.
 */

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn run() {}
