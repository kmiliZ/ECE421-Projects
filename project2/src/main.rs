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
}

impl TreeNode {
    fn new(z: u32) -> Self {
        Self {
            color: NodeColor::Red,
            key: z,
            parent: None,
            left: None,
            right: None,
        }
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
        match child.as_ref().borrow().parent {
            Some(ref parent) => {
                if parent.clone().as_ref().borrow().color == NodeColor::Black {
                    // no fixing needed;
                    return;
                } else {
                    match parent.as_ref().borrow().parent {
                        Some(ref grandp) => {
                            //check uncle.
                            // if parent.clone().as_ref().borrow().key
                            //     > grandp.clone().as_ref().borrow().key
                            // {}
                            // if TreeNode::is_greater(grandp, parent.clone().as_ref().borrow().key) {}
                            // match &grandp.as_ref().borrow().parent
                            match grandp.as_ref().borrow().left {
                                Some(ref grandp) => {
                                    //check uncle.
                                    // if parent.clone().as_ref().borrow().key
                                    //     > grandp.clone().as_ref().borrow().key
                                    // // {}
                                    // if TreeNode::is_greater(grandp, parent.clone().as_ref().borrow().key) {}
                                    // // match &grandp.as_ref().borrow().parent
                                    // match grandp.as_ref().borrow().parent
                                }
                                None => {
                                    // child is the root node
                                    child.as_ref().borrow_mut().color = NodeColor::Black;
                                }
                            }
                        }
                        None => {
                            // child is the root node
                            child.as_ref().borrow_mut().color = NodeColor::Black;
                        }
                    };
                }
            }
            None => {
                println!("No parent")
            }
        }
    }

    fn insert(node: &mut Option<Rc<RefCell<TreeNode>>>, key: u32) {
        let new_leaf: Option<Rc<RefCell<TreeNode>>> = {
            let mut return_leaf: Option<Rc<RefCell<TreeNode>>> = None;
            if let Some(current_node) = node {
                println!("non-empty tree");
                // compare with the tree root node with key
                if TreeNode::is_greater(current_node, key) {
                    let mut tNode = current_node.borrow_mut();
                    if !tNode.right.is_none() {
                        Self::insert(&mut tNode.right, key);
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
                        Self::insert(&mut tNode.left, key);
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
            } else {
                return_leaf = None;
            }
            return_leaf
        };
        if let Some(leaf) = new_leaf {
            println!("need fix");
            Self::fix(&leaf);
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

            println!("value:{}, color:{}", current_key, current_color);
            if let Some(left_node) = &(*current_node).as_ref().borrow().left {
                let left_key = left_node.as_ref().borrow().key;
                let mut left_color = "";

                if left_node.as_ref().borrow().color == NodeColor::Black {
                    left_color = "Black";
                } else {
                    left_color = "Red";
                }
                println!(
                    "{} left node:{} with color {}",
                    current_key, left_key, left_color
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
            TreeNode::insert(&mut self.root, key);
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
    RedBlackTree::tree_insert(&mut tree, 13);
    RedBlackTree::tree_insert(&mut tree, 1);
    RedBlackTree::tree_insert(&mut tree, 14);
    RedBlackTree::tree_insert(&mut tree, 12);

    TreeNode::print_tree(&tree.root);
}
