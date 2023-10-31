use std::rc::Rc;
use std::cell::RefCell;

pub type WrappedTreeNode = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: WrappedTreeNode,
    pub right: WrappedTreeNode,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}
