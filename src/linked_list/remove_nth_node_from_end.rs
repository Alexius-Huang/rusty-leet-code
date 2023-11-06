use super::linked_list::{WrappedListNode, ListNode};

/**
 *  Problem 19. Remove nth Node from End of List
 *  See: https://leetcode.com/problems/remove-nth-node-from-end-of-list/
 * 
 *  Given the head of a linked list, remove the nth node from the end of
 *  the list and return its head.
 */
pub fn run(head: WrappedListNode, n: i32) -> WrappedListNode {
    // if head.is_none() { return None; }

    // let mut nodes: Vec<Box<ListNode>> = vec![];

    // let mut len = 0;
    // let mut cur_node = head.unwrap();
    // while let Some(node) = cur_node.next.take() {
    //     len += 1;
    //     let next_node = node;
    //     // nodes.push(node);
    // }

    head
}

#[cfg(test)]
mod test {
    use crate::linked_list::linked_list::ListNode;
    use super::*;

    #[ignore = "TODO: Complete!"]
    #[test]
    fn it_removes_nth_element_from_end_of_list() {
        let list = ListNode::build(vec![1, 2, 3, 4, 5]);
        
        assert_eq!(
            run(Some(Box::new(list)), 2),
            Some(Box::new(ListNode::build(vec![1, 2, 3, 5])))
        );
    }
}