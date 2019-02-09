use super::helper::TreeNode;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

// 401
pub fn read_binary_watch(num: i32) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    fn bit_count(num: i32) -> i32 {
        format!("{:b}", num)
            .chars()
            .filter(|c| *c == '1')
            .collect::<Vec<char>>()
            .len() as i32
    }

    for h in 0..12 {
        for m in 0..60 {
            if bit_count(h * 64 + m) == num {
                result.push(format!("{}:{:02}", h, m));
            }
        }
    }

    result
}

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

// 409
pub fn longest_palindrome(s: String) -> i32 {
    let mut store = HashSet::new();
    let mut result = 0;

    for c in s.chars() {
        if store.contains(&c) {
            store.remove(&c);
            result += 1;
        } else {
            store.insert(c);
        }
    }

    if store.is_empty() {
        result * 2
    } else {
        result * 2 + 1
    }
}
