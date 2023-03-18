use text_io::read;

mod avl;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Clone, Debug, PartialEq)]

enum NodeColor {
    Red,
    Black,
}
#[derive(Debug)]
struct RedBlackTree {
    pub root: Option<Rc<RefCell<TreeNode>>>,
}
#[derive(Debug)]
struct TreeNode {
    pub color: NodeColor,
    pub key: u32,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
    height: i32,
}

impl TreeNode {
    fn new(z: u32) -> Self {
        Self {
            color: NodeColor::Red,
            key: z,
            parent: None,
            left: None,
            right: None,
            height: 1,
        }
    }

    fn height(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        node.as_ref().map_or(0, |n| n.as_ref().borrow().height)
    }

    fn update_height(node: &Option<Rc<RefCell<TreeNode>>>) {
        // Check heights of left and right node, take the larger one and add 1.
        let height = std::cmp::max(
            Self::height(&node.as_ref().unwrap().as_ref().borrow().left),
            Self::height(&node.as_ref().unwrap().as_ref().borrow().right),
        ) + 1;
        node.as_ref().unwrap().borrow_mut().height = height;
    }
    // fn is_left_child(parent: &Rc<RefCell<TreeNode>>) -> bool {
    //     match parent.as_ref().borrow().left {
    //         Some(left) => {

    //         }
    //         None => false,
    //     }
    // }

    fn change_colour(node: &mut TreeNode, color: NodeColor) {
        node.color = color;
    }
    fn is_greater(node: &Rc<RefCell<TreeNode>>, z: u32) -> bool {
        if node.as_ref().borrow().key < z {
            return true;
        }
        false
    }

    fn fix(child: &Rc<RefCell<TreeNode>>) {
        println!("Fixing");
        match child.as_ref().borrow().parent {
            Some(ref parent) => {
                if parent.clone().as_ref().borrow().color == NodeColor::Black {
                    // no fixing needed;
                    println!("FIX DEBUG: node's parent is black, no fixing needed");
                    return;
                } else {
                    match parent.as_ref().borrow().parent {
                        Some(ref grandp) => {
                            // check uncle.

                            if TreeNode::is_greater(grandp, parent.clone().as_ref().borrow().key) {
                                // parent was the right child
                                println!("FIX DEBUG: uncle on the left");
                                match grandp.as_ref().borrow().left {
                                    Some(ref uncle) => {
                                        if uncle.clone().as_ref().borrow().color == NodeColor::Red {
                                            // recolor;
                                            // parent.as_ref().borrow_mut().color = NodeColor::Black;
                                            // uncle.as_ref().borrow_mut().color = NodeColor::Black;
                                            // grandp.as_ref().borrow_mut().color = NodeColor::Red;
                                            Self::fix(grandp);
                                        } else {
                                            // rotate
                                        }
                                    }
                                    None => {
                                        // rotate
                                    }
                                }
                            } else {
                                println!("FIX DEBUG: uncle on the right");

                                match grandp.as_ref().borrow().right {
                                    Some(ref uncle) => {
                                        if uncle.clone().as_ref().borrow().color == NodeColor::Red {
                                            // recolor;
                                            // parent.as_ref().borrow_mut().color = NodeColor::Black;
                                            // uncle.as_ref().borrow_mut().color = NodeColor::Black;
                                            // grandp.as_ref().borrow_mut().color = NodeColor::Red;
                                            Self::fix(grandp);
                                        }
                                    }
                                    None => {
                                        // rotate
                                    }
                                }
                            }
                        }
                        None => {
                            println!("FIX DEBUG: Child has no grand parent");
                        }
                    };
                }
            }
            None => {
                println!("FIX DEBUG: Child has no parent");
                // child is the root node
                child.as_ref().borrow_mut().color = NodeColor::Black;
            } // case is it the root => update color
              // case rotation: needs
        }
    }

    fn insert(node: &mut Option<Rc<RefCell<TreeNode>>>, key: u32) -> Option<Rc<RefCell<TreeNode>>> {
        let new_leaf: Option<Rc<RefCell<TreeNode>>> = {
            let mut return_leaf: Option<Rc<RefCell<TreeNode>>> = None;
            if let Some(current_node) = node {
                // compare with the tree root node with key
                if TreeNode::is_greater(current_node, key) {
                    let mut tNode = current_node.borrow_mut();
                    if !tNode.right.is_none() {
                        return_leaf = Self::insert(&mut tNode.right, key);
                    } else {
                        println!("inserted right node");
                        let mut new_node = TreeNode::new(key);
                        new_node.parent = Some(current_node.clone());
                        let new_leaf = Rc::new(RefCell::new(new_node));
                        tNode.right = Some(new_leaf.clone());
                        return_leaf = Some(new_leaf.clone());
                        // Self::fix(&new_leaf);
                    }
                } else {
                    let mut tNode = current_node.borrow_mut();
                    if !tNode.left.is_none() {
                        return_leaf = Self::insert(&mut tNode.left, key);
                    } else {
                        println!("inserted left node");
                        let mut new_node = TreeNode::new(key);
                        new_node.parent = Some(current_node.clone());
                        let new_leaf = Rc::new(RefCell::new(new_node));
                        tNode.left = Some(new_leaf.clone());
                        return_leaf = Some(new_leaf.clone());
                        // Self::fix(&new_leaf);
                    }
                }

                let height = std::cmp::max(
                    Self::height(&current_node.as_ref().borrow().left),
                    Self::height(&current_node.as_ref().borrow().right),
                ) + 1;
                current_node.borrow_mut().height = height;
            } else {
                return_leaf = None;
            }
            return_leaf
        };

        return new_leaf;
    }

    fn node_insert(node: &mut Option<Rc<RefCell<TreeNode>>>, key: u32) {
        let leaf_node = Self::insert(node, key);
        match leaf_node {
            Some(child) => {
                Self::fix(&child);
                // println!(
                //     "new_leaf node return with value:{}",
                //     child.as_ref().borrow().key
                // );
            }
            None => {
                eprintln!("new_leaf node return with value: None");
            }
        }
    }

    fn print_tree(node: &Option<Rc<RefCell<TreeNode>>>) {
        if let Some(current_node) = &node {
            let current_key = current_node.as_ref().borrow().key;
            let mut current_color = "";

            if current_node.as_ref().borrow().color == NodeColor::Black {
                current_color = "Black";
            } else {
                current_color = "Red";
            }

            println!(
                "value:{}, color:{}, height:{}",
                current_key,
                current_color,
                current_node.as_ref().borrow().height
            );
            if let Some(left_node) = &(*current_node).as_ref().borrow().left {
                let left_key = left_node.as_ref().borrow().key;
                let mut left_color = "";

                if left_node.as_ref().borrow().color == NodeColor::Black {
                    left_color = "Black";
                } else {
                    left_color = "Red";
                }
                println!(
                    "{} left node:{} with color {},with height{}",
                    current_key,
                    left_key,
                    left_color,
                    left_node.as_ref().borrow().height
                );
                Self::print_tree(&(*current_node).as_ref().borrow().left);
            } else {
                println!("{} left node is empty", current_key);
            }
            if let Some(right_node) = &(*current_node).as_ref().borrow().right {
                let right_key = right_node.as_ref().borrow().key;
                let mut right_color = "";

                if right_node.as_ref().borrow().color == NodeColor::Black {
                    right_color = "Black";
                } else {
                    right_color = "Red";
                }
                println!(
                    "{} right node:{} with color {}",
                    current_key, right_key, right_color
                );
                Self::print_tree(&(*current_node).as_ref().borrow().right);
            } else {
                println!("{} right node is empty", current_key);
            }
        } else {
            println!("empty node");
        }
    }
}

impl RedBlackTree {
    fn new() -> RedBlackTree {
        RedBlackTree { root: None }
    }
    fn clone(&self) -> Self {
        if let Some(node) = &self.root {
            return RedBlackTree {
                root: Some(Rc::clone(node)),
            };
        } else {
            return RedBlackTree { root: None };
        }
    }

    fn tree_insert(&mut self, key: u32) {
        if let Some(ref mut current_node) = self.root {
            // have a node already
            println!("insert another node!");

            TreeNode::node_insert(&mut self.root, key);
        } else {
            // case x is the root
            println!("insert root node!");
            let mut new_node = TreeNode::new(key);
            TreeNode::change_colour(&mut new_node, NodeColor::Black);
            let rc = Rc::new(RefCell::new(new_node));
            *self = RedBlackTree {
                root: Some(rc.clone()),
            };
        }
    }

    // fix should be called after we inserted a leaf node
    // deal with the error
}

fn main() {
    // let mut tree: RedBlackTree = RedBlackTree::new();

    // println!("do you want to insert?");

    // // let line: String = read!("{}\n");
    // // println!("{}", line);
    // (&mut tree).insert(12);
    // println!("tree after inserting 12: {:#?}", tree);
    // (&mut tree).insert(1);
    // (&mut tree).insert(13);
    // (&mut tree).insert(0);

    // tree.print_tree();
    let mut tree = RedBlackTree::new();
    println!("          Insert 13");
    RedBlackTree::tree_insert(&mut tree, 13);
    println!("          Insert 1");

    RedBlackTree::tree_insert(&mut tree, 1);
    println!("          Insert 2");

    RedBlackTree::tree_insert(&mut tree, 2);
    RedBlackTree::tree_insert(&mut tree, 3);
    RedBlackTree::tree_insert(&mut tree, 4);

    println!("          Insert 15");

    RedBlackTree::tree_insert(&mut tree, 15);
    RedBlackTree::tree_insert(&mut tree, 16);

    // TreeNode::print_tree(&tree.root);
}
