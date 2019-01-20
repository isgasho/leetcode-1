use super::helper::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

// 226
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        None => None,
        Some(n) => {
            let left = invert_tree(n.borrow().left.clone());
            let right = invert_tree(n.borrow().right.clone());

            n.borrow_mut().left = right;
            n.borrow_mut().right = left;

            return Some(n);
        }
    }
}
