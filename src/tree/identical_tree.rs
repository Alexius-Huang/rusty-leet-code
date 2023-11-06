use super::tree::WrappedTreeNode;

/**
 *  Problem 100. Same Tree (Easy)
 *  See: https://leetcode.com/problems/same-tree/ 
 * 
 *  Given the roots of two binary trees p and q, write a function to check if
 *  they are the same or not.
 *
 *  Two binary trees are considered the same if they are structurally identical,
 *  and the nodes have the same value.
 */
pub fn run(p: WrappedTreeNode, q: WrappedTreeNode) -> bool {
    fn is_identical(p: &WrappedTreeNode, q: &WrappedTreeNode) -> bool {
        match (p.is_none(), q.is_none()) {
            (true, true) => true,
            (true, false) | (false, true) => false,
            _ => {
                let (p_node, q_node) = (
                    p.as_ref().unwrap().borrow(),
                    q.as_ref().unwrap().borrow()
                );
    
                let (p_l_node, p_r_node) = (&p_node.left, &p_node.right);
                let (q_l_node, q_r_node) = (&q_node.left, &q_node.right);
    
                p_node.val == q_node.val
                    && is_identical(p_l_node, q_l_node)
                    && is_identical(p_r_node, q_r_node)
            }
        }
    }

    is_identical(&p, &q)
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore = "TODO: implement test"]
    #[test]
    fn it_compares_if_both_trees_are_identical() {

    }
}