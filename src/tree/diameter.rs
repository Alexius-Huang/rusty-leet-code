use super::tree::WrappedTreeNode;

/**
 *  Problem 543. Diameter of a Binary Tree (Easy)
 *  See: https://leetcode.com/problems/diameter-of-binary-tree/ 
 * 
 *  Given the root of a binary tree, return the length of the diameter
 *  of the tree.
 *
 *  The diameter of a binary tree is the length of the longest path
 *  between any two nodes in a tree.
 * 
 *  This path may or may not pass through the root.
 *  
 *  The length of a path between two nodes is represented by the number
 *  of edges between them.
 */
pub fn run(root: WrappedTreeNode) -> i32 {
    // if root.is_none() { return 0; }

    // // Returns (max depth, diameter of child)
    // fn recursive(root: &WrappedTreeNode) -> (i32, i32) {
    //     if root.is_none() { return (0, 0); }
    //     let root_node = root.as_ref().unwrap().borrow();

    //     match (&root_node.left.is_none(), &root_node.right.is_none()) {
    //         (true, true) => return (0, 0),
    //         (true, false) | (false, true) => return (1, 1),
    //         _ => ()
    //     }

    //     let (l, r) = (&root_node.left, &root_node.right);
    //     let (
    //         (l_depth, l_diameter),
    //         (r_depth, r_diameter)
    //     ) = (recursive(l), recursive(r));

    //     let total_diameter = l_depth + r_depth + 2;

    //     let max_depth = (if l_depth > r_depth { l_depth } else { r_depth }) + 1;


    //     (max_depth, diameter)
    // }

    0
}

// fn max_depth(root: &WrappedTreeNode) -> i32 {
//     if root.is_none() { return 0; }
//     let root_node = root.as_ref().unwrap().borrow();
//     let (left, right) = (&root_node.left, &root_node.right);

//     let (l_depth, r_depth) = (max_depth(&left), max_depth(&right));

//     (if l_depth > r_depth { l_depth } else { r_depth }) + 1
// }

// left max depth / left diameter
// right max depth / right diameter
// 
