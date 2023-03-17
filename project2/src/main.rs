use text_io::read;

mod avl;
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

    fn change_colour(node: &mut TreeNode, color: NodeColor) {
        node.color = color;
    }
    fn is_greater(node: &mut Rc<RefCell<TreeNode>>, z: u32) -> bool {
        if node.borrow().key < z {
            return true;
        }
        false
    }

    fn fix(node: &Option<Rc<RefCell<TreeNode>>>) {
        match &node {
            Some(current) => match &current.borrow().parent {
                Some(parent) => if parent.borrow().color == NodeColor::Black {},
                None => {
                    println!("fsdjl")
                }
            },
            None => {
                println!("fsdjl")
            }
        }
    }

    fn insert(node: &mut Option<Rc<RefCell<TreeNode>>>, key: u32) {
        if let Some(ref mut current_node) = &node {
            println!("non-empty tree");
            // compare with the tree root node with key
            //recusivly call insertion
            if TreeNode::is_greater(current_node, key) {
                let right_child = &mut current_node.borrow_mut().right;
                if !right_child.is_none() {
                    Self::insert(node, key);
                } else {
                    println!("inserted left node");
                    let mut new_node = TreeNode::new(key);
                    new_node.parent = Some(Rc::clone(current_node));
                    right_child = &mut Some(Rc::new(RefCell::new(new_node)));

                    Self::fix(right_child);
                }
            } else {
                let left_child = &mut current_node.borrow_mut().left;
                if !left_child.is_none() {
                    left_child.insert(key);
                } else {
                    println!("inserted left node");

                    // the case the left side is empty
                    let mut new_node = TreeNode::new(key);
                    new_node.parent.root = Some(Rc::clone(current_node));
                    left_child.root = Some(Rc::new(RefCell::new(new_node)));

                    Self::fix(left_child);
                }
            }
        } else {
            // case x is the root
            println!("root node!");
            let mut new_node = TreeNode::new(key);
            TreeNode::change_colour(&mut new_node, NodeColor::Black);
            let rc = Rc::new(RefCell::new(new_node));
            *self = RedBlackTree {
                root: Some(rc.clone()),
            };
        }
        Self::fix(&node);
    }

    fn print_tree(&self) {
        // if let Some(current_node) = &self.root {
        //     let current_key = current_node.borrow().key;
        //     let mut current_color = "";

        //     if current_node.borrow().color == NodeColor::Black {
        //         current_color = "Black";
        //     } else {
        //         current_color = "Red";
        //     }

        //     println!("value:{}, color:{}", current_key, current_color);
        //     if let Some(left_node) = &(*current_node).borrow().left.root {
        //         let left_key = left_node.borrow().key;
        //         let mut left_color = "";

        //         if left_node.borrow().color == NodeColor::Black {
        //             left_color = "Black";
        //         } else {
        //             left_color = "Red";
        //         }
        //         println!(
        //             "{} left node:{} with color {}",
        //             current_key, left_key, left_color
        //         );
        //         Self::print_tree(&(*current_node).borrow().left);
        //     } else {
        //         println!("{} left node is empty", current_key);
        //     }
        //     if let Some(right_node) = &(*current_node).borrow().right.root {
        //         let right_key = right_node.borrow().key;
        //         let mut right_color = "";

        //         if right_node.borrow().color == NodeColor::Black {
        //             right_color = "Black";
        //         } else {
        //             right_color = "Red";
        //         }
        //         println!(
        //             "{} right node:{} with color{}",
        //             current_key, right_key, right_color
        //         );
        //         Self::print_tree(&(*current_node).borrow().right);
        //     } else {
        //         println!("{} right node is empty", current_key);
        //     }
        // } else {
        //     println!("empty node");
        // }
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
    let mut root_node = TreeNode::new(12);
    TreeNode::insert(&mut Some(Rc::new(RefCell::new(root_node))), 12);

    // println!("{:#?}", *root_node);

    // println!("12 1 {:#?}", tree);

    // println!("12 1 13{:#?}", tree);*/

    // let mut tree = avl::AVLTree::new();
    // tree.insert(10);
    // tree.insert(5);

    // println!("Tree: {:#?}", tree);
}
