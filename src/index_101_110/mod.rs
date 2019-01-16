use super::helper::TreeNode;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

// 104
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(n) => {
            let left = n.borrow().left.clone();
            let right = n.borrow().right.clone();

            if left != None && right != None {
                return cmp::max(max_depth(left), max_depth(right)) + 1;
            } else if left != None {
                return max_depth(left) + 1;
            } else {
                return max_depth(right) + 1;
            }
        }
    }
}
