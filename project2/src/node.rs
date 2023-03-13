use text_io::read;

use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone, Debug, PartialEq)]

enum NodeColor {
    Red,
    Black,
}
type Tree = Rc<RefCell<TreeNode<u32>>>;
type RedBlackTree = Option<Tree>;
struct TreeNode<T> {
    pub color: NodeColor,
    pub key: T,
    pub parent: RedBlackTree,
    left: RedBlackTree,
    right: RedBlackTree,
}

impl<T> TreeNode<T> {
    fn new(z: T) -> Self {
        Self {
            color: NodeColor::Red,
            key: z,
            parent: None,
            left: None,
            right: None,
        }
    }
    // pub fn insert<T>(mut self, z: T) {}
}
fn insert(tree: &mut RedBlackTree, key: u32) {
    if let Some(ref mut node) = tree {
        println!("insert!");
    } else {
        println!("insert!");
        let mut new_node = TreeNode::new(key);
        let rc = Rc::new(RefCell::new(new_node));
        *tree = Some(rc.clone());
    }
}
fn print_tree(tree: &mut RedBlackTree) {
    let mut node = tree.clone();
    if let Some(n) = node {
        unsafe {
            let b = &*n.as_ptr();
            println!("tree:{}", b.key);
        }
    } else {
        println!("empty tree");
    }
}
