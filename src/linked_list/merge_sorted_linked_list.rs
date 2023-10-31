use std::mem;

use super::linked_list::{ListNode, WrappedListNode};

/**
*  Problem 21. Merge Two Sorted Linked List
*  See: https://leetcode.com/problems/merge-two-sorted-lists/
*
*  You are given the heads of two sorted linked lists list1 and list2.

*  Merge the two lists into one sorted list. The list should be made
*  by splicing together the nodes of the first two lists.
*
*  Return the head of the merged linked list.
*/
pub fn merge(mut list1: WrappedListNode, list2: WrappedListNode) -> WrappedListNode {
    if list1.is_none() {
        return list2;
    }
    if list2.is_none() {
        return list1;
    }

    let mut cur_list1_node = list1.as_mut()?;
    let mut cur_list2_node = list2.as_ref()?;

    loop {
        if cur_list1_node.val < cur_list2_node.val {
            if cur_list1_node.next.is_none() {
                break;
            }
            cur_list1_node = cur_list1_node.next.as_mut()?;
            continue;
        }

        let list1_next = mem::replace(
            &mut cur_list1_node.next, 
            ListNode::wrap(cur_list1_node.val)
        );
        cur_list1_node.val = cur_list2_node.val;

        cur_list1_node = cur_list1_node.next.as_mut()?;
        cur_list1_node.next = list1_next;

        match cur_list2_node.next.as_ref() {
            Some(node) => {
                cur_list2_node = node;
            }
            None => {
                return list1;
            }
        }
    }

    // When list1 is at the end, we join the rest of list2
    cur_list1_node.next = ListNode::wrap(cur_list2_node.val);
    cur_list1_node = cur_list1_node.next.as_mut()?;

    while let Some(node) = cur_list2_node.next.as_ref() {
        cur_list1_node.next = ListNode::wrap(node.val);
        cur_list1_node = cur_list1_node.next.as_mut()?;

        cur_list2_node = node;
    }

    list1
}

#[cfg(test)]
mod test {
    use super::super::linked_list::ListNode;
    use super::*;

    #[test]
    fn it_merges_two_linked_list() {
        let list1 = ListNode::build(vec![1, 2, 4]);
        let list2 = ListNode::build(vec![1, 3, 4]);
        let result = ListNode::build(vec![1, 1, 2, 3, 4, 4]);

        assert_eq!(
            merge(Some(Box::new(list1)), Some(Box::new(list2))),
            Some(Box::new(result))
        );

        let list1 = ListNode::build(vec![3, 5, 7]);
        let list2 = ListNode::build(vec![1, 2, 4, 6]);
        let result = ListNode::build(vec![1, 2, 3, 4, 5, 6, 7]);

        assert_eq!(
            merge(Some(Box::new(list1)), Some(Box::new(list2))),
            Some(Box::new(result))
        );

        let list1 = ListNode::build(vec![1, 3, 5]);
        let list2 = ListNode::build(vec![2, 4, 5, 6]);
        let result = ListNode::build(vec![1, 2, 3, 4, 5, 5, 6]);

        assert_eq!(
            merge(Some(Box::new(list1)), Some(Box::new(list2))),
            Some(Box::new(result))
        );
    }
}
