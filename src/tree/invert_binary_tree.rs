use super::tree::WrappedTreeNode;

/**
 *  Problem 226. Invert Binary Tree (Easy)
 *  See: https://leetcode.com/problems/invert-binary-tree/
 *
 *  Given the root of a binary tree, invert the tree, and return its root.
 */
pub fn recursive_inversion(root: WrappedTreeNode) -> WrappedTreeNode {
    if root.is_none() { return None; }

    let mut tree_node = root.as_ref()?.borrow_mut();

    let (left, right) = (tree_node.left.take(), tree_node.right.take());
    tree_node.left = recursive_inversion(right);
    tree_node.right = recursive_inversion(left);

    drop(tree_node);
    root
}

#[cfg(test)]
mod test {

    #[ignore = "Implement binary tree test"]
    #[test]
    fn it_inverts_the_binary_tree() {
        assert!(true);
    }
}
