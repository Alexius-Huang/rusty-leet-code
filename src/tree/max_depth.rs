use super::tree::WrappedTreeNode;

/**
 *  Problem 104. Max Depth of Binary Tree (Easy)
 *  See: https://leetcode.com/problems/maximum-depth-of-binary-tree/
 * 
 *   Given the root of a binary tree, return its maximum depth.
 *
 *   A binary tree's maximum depth is the number of nodes along the
 *   longest path from the root node down to the farthest leaf node.
 */
pub fn run_iteratively(root: WrappedTreeNode) -> i32 {
    if root.is_none() { return 0; }

    let node = root.as_ref().unwrap().borrow();
    let (left, right) = (node.left.clone(), node.right.clone());

    let (left_depth, right_depth) = (run_iteratively(left), run_iteratively(right));

    (if left_depth > right_depth { left_depth } else { right_depth }) + 1
}

#[cfg(test)]
mod test {

    #[ignore = "Implement binary tree test"]
    #[test]
    fn it_calculates_the_max_depth_of_tree() {}
}
