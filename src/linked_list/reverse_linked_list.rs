use super::linked_list::WrappedListNode;

/**
 *  Problem 206. Reversed Linked List (Easy)
 *  See: https://leetcode.com/problems/reverse-linked-list/
 *
 *  Given the head of a singly linked list, reverse the list, and
 *  return the reversed list.
 */
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
    use super::super::linked_list::ListNode;
    use super::*;

    #[test]
    fn it_reverses_linked_list() {
        let input = ListNode::build(vec![1, 2, 3, 4, 5]);

        let result = ListNode::build(vec![5, 4, 3, 2, 1]);

        assert_eq!(
            run_iteratively(Some(Box::new(input))),
            Some(Box::new(result))
        );
    }
}
