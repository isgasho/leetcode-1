use super::helper::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

// 405
pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(x) => {
            let left = x.borrow().left.clone();
            let right = x.borrow().right.clone();

            if left != None {
                return left.clone().unwrap().borrow().val
                    + sum_of_left_leaves(left)
                    + sum_of_left_leaves(right);
            } else {
                return sum_of_left_leaves(right);
            }
        }
    }
}
