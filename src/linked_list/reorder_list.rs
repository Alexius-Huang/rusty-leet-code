use super::linked_list::{WrappedListNode, ListNode};

/**
 *  Problem 143. Reorder List (Medium)
 *  See: https://leetcode.com/problems/reorder-list
 *  
 *  You are given the head of a singly linked-list.
 * 
 *  The list can be represented as:
 *
 *  > L0 → L1 → … → Ln - 1 → Ln
 *  
 *  Reorder the list to be on the following form:
 *  
 *  > L0 → Ln → L1 → Ln - 1 → L2 → Ln - 2 → …
 *  
 *  You may not modify the values in the list's nodes.
 * 
 *  Only nodes themselves may be changed.
 */
pub fn run(head: &mut WrappedListNode) {
    if head.is_none() { return (); }

    // Convert nodes other than head into array of nodes
    let mut next_node = head.as_mut().unwrap().next.take();
    let mut nodes = vec![];
    while let Some(mut node) = next_node {
        let next_next_node = node.next.take();
        nodes.push(Some(Box::new(ListNode { val: node.val, next: None })));
        next_node = next_next_node;
    }

    // Split nodes into proper left/right section of slices
    let left_slice;
    let right_slice;

    let len = nodes.len();
    let mid = len / 2;
    let mut last_node: WrappedListNode = None;
    if len % 2 == 0 {
        left_slice = &nodes[0..mid];
        right_slice = &nodes[mid..len];
    } else {
        left_slice = &nodes[0..mid];
        right_slice = &nodes[(mid + 1)..len];
        last_node = nodes[mid].clone();
    }

    // Populate the list with reordering
    let mut cur_node = head.as_mut().unwrap();
    for i in 0..mid {
        cur_node.next = right_slice[mid - i - 1].clone();
        cur_node = cur_node.next.as_mut().unwrap();

        cur_node.next = left_slice[i].clone();
        cur_node = cur_node.next.as_mut().unwrap();
    }

    cur_node.next = last_node;
}

#[cfg(test)]
mod test {
    use super::super::linked_list::ListNode;
    use super::*;

    #[test]
    fn it_reorders_the_list() {
        let mut list = Some(Box::new(ListNode::build(vec![1, 2, 3, 4])));
        run(&mut list);
        assert_eq!(
            list,
            Some(Box::new(ListNode::build(vec![1, 4, 2, 3])))
        );

        let mut list = Some(Box::new(ListNode::build(vec![1, 2, 3, 4, 5])));
        run(&mut list);
        assert_eq!(
            list,
            Some(Box::new(ListNode::build(vec![1, 5, 2, 4, 3])))
        );
    }
}
