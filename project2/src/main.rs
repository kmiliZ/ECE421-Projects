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

impl<T: std::cmp::PartialOrd> TreeNode<T> {
    fn new(z: T) -> Self {
        Self {
            color: NodeColor::Red,
            key: z,
            parent: None,
            left: None,
            right: None,
        }
    }
    fn is_greater(node: &mut TreeNode<T>, z: T) -> bool {
        if node.key < z {
            return true;
        }
        false
    }
    // pub fn insert<T>(mut self, z: T) {}
}
fn insert(tree: &mut RedBlackTree, key: u32) {
    if let Some(node) = tree {
        println!("insert!");
        let a = node.clone().as_ptr();
        // if TreeNode::is_greater(&*a, key) {
        //     println!("insert rights");
        // }
    } else {
        println!("root node!");
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
            if let Some(left) = &b.borrow().left {
                print_tree(&mut Some(left.clone()));
            }
            // if let Some(right) = b.right {
            //     print_tree(&mut Some(right));
            // }
        }
    } else {
        println!("empty node");
    }
}

fn main() {
    let mut tree: RedBlackTree = None;

    print_tree(&mut tree);

    println!("do you want to insert?");

    let line: String = read!("{}\n");
    // println!("{}", line);
    insert(&mut tree, 12);
    print_tree(&mut tree);
    insert(&mut tree, 12);

    println!("Hello, world!");
}
