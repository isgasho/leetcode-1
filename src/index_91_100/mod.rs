use super::helper::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

// 100
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if p == None && q == None {
        return true;
    }
    if p == None || q == None {
        return false;
    }

    let p = p.unwrap();
    let q = q.unwrap();
    let p_tree = p.borrow();
    let q_tree = q.borrow();
    let p_tree_left = p.borrow().left.clone();
    let q_tree_left = q.borrow().left.clone();
    let p_tree_right = p.borrow().right.clone();
    let q_tree_right = q.borrow().right.clone();

    if p_tree.val != q_tree.val {
        return false;
    }
    return is_same_tree(p_tree_left, q_tree_left) && is_same_tree(p_tree_right, q_tree_right);
}
