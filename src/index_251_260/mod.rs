use super::helper::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

// 257
pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    fn helper(root: Option<Rc<RefCell<TreeNode>>>, middle: String, res: &mut Vec<String>) {
        if root == None {
            return;
        }

        match root {
            None => return,
            Some(x) => {
                let left = x.borrow().left.clone();
                let right = x.borrow().right.clone();

                if left == None && right == None {
                    res.push(middle.clone() + &x.borrow().val.to_string());
                }
                helper(
                    left,
                    middle.clone() + &x.borrow().val.to_string() + &"->",
                    res,
                );
                helper(
                    right,
                    middle.clone() + &x.borrow().val.to_string() + &"->",
                    res,
                );
            }
        }
    }

    helper(root, "".to_string(), &mut result);

    return result;
}

// 258
pub fn add_digits(num: i32) -> i32 {
    (num - 1) % 9 + 1
}
