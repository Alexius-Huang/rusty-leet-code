use std::cmp;

use super::tree::WrappedTreeNode;

/**
 *  Problem 110. Balanced Binary Tree (Easy)
 *  See: https://leetcode.com/problems/balanced-binary-tree/
 * 
 *  Given a binary tree, determine if it is height-balanced.
 * 
 *  A height-balanced binary tree is a binary tree in which the depth
 *  of the two subtrees of every node never differs by more than one.
 *
 */

// FAILED
pub fn run(root: WrappedTreeNode) -> bool {
    if root.is_none() { return true; }

    let node = root.as_ref().unwrap().borrow();
    let (l_node, r_node) = (&node.left, &node.right);

    fn height_of_tree(root: &WrappedTreeNode) -> i32 {
        if root.is_none() { return 0; }
        
        let node = root.as_ref().unwrap().borrow();
        let (l_node, r_node) = (&node.left, &node.right);
    
        match (l_node.is_none(), r_node.is_none()) {
            (true, true) => 1,
            (true, false) => height_of_tree(&l_node) + 1,
            (false, true) => height_of_tree(&r_node) + 1,
            _ => cmp::max(height_of_tree(&l_node), height_of_tree(&r_node)) + 1
        }
    }
    
    fn is_balanced(l_node: &WrappedTreeNode, r_node: &WrappedTreeNode) -> bool {
        match (l_node.is_none(), r_node.is_none()) {
            (true, true) => true,
            (true, false) => height_of_tree(l_node) < 2,
            (false, true) => height_of_tree(r_node) < 2,
            _ => {
                let l_node_unwrapped = l_node.as_ref().unwrap().borrow();
                let (l_l_node, l_r_node) = (&l_node_unwrapped.left, &l_node_unwrapped.right);
                let r_node_unwrapped = r_node.as_ref().unwrap().borrow();
                let (r_l_node, r_r_node) = (&r_node_unwrapped.left, &r_node_unwrapped.right);

                let (l_height, r_height) = (height_of_tree(l_node), height_of_tree(r_node));

                is_balanced(&l_l_node, &l_r_node)
                    && is_balanced(&r_l_node, &r_r_node)
                    && i32::abs(l_height - r_height) < 2
            }
        }
    }

    is_balanced(l_node, r_node)
}
