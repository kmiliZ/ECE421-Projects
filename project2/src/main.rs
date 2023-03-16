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
    pub root: Option<Rc<RefCell<TreeNode<u32>>>>,
}
#[derive(Debug)]
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
            parent: RedBlackTree::new(),
            left: RedBlackTree::new(),
            right: RedBlackTree::new(),
        }
    }

    fn change_colour(node: &mut TreeNode<T>, color: NodeColor) {
        node.color = color;
    }
    fn is_greater(node: &mut Rc<RefCell<TreeNode<T>>>, z: T) -> bool {
        if node.borrow().key < z {
            return true;
        }
        false
    }
}

impl RedBlackTree {
    fn new() -> RedBlackTree {
        RedBlackTree { root: None }
    }

    // fix should be called after we inserted a leaf node
    // deal with the error
    fn fix(self: &mut RedBlackTree) {
        // let's see if this tree violates the rules or not
        if let Some(ref mut current) = self.root {
            if let Some(ref mut parent) = current.borrow_mut().parent.root {
                let mut cloned_parent = Rc::clone(parent);
                // let color = &mut cloned_parent.borrow_mut().color;
                if Rc::clone(parent).borrow_mut().color == NodeColor::Red {
                    // if let Some(ref mut grandparent) = parent.borrow_mut().parent.root {
                    //     if let Some(ref mut uncle) = grandparent.borrow_mut().parent.root {
                    //         if uncle.borrow_mut().color == NodeColor::Red {
                    //             println!("DEBUG MESSAGE: Uncle is RED, peform RECOLOR");
                    //             parent.borrow_mut().color = NodeColor::Black;
                    //             uncle.borrow_mut().color = NodeColor::Black;
                    //             grandparent.borrow_mut().color = NodeColor::Red;
                    //             Self::fix(&mut parent.borrow_mut().parent); // fix grand parent
                    //         } else {
                    //             // uncle is black
                    //             // TODO: Rotation here depending on the case
                    //         }
                    //     } else {
                    //         // uncle is black
                    //         // TODO: Rotation here
                    //     }
                    // } else {
                    //     println!("DEBUG MESSAGE: ERROR: I don't think this line should be printed. this means the the root node now has color red");
                    // }
                }
            } else {
                // we reached the root node, turn node to black
                current.borrow_mut().color = NodeColor::Black;
                return;
            }
        } else {
            // nll node
            return;
        }
        // get current node's uncle

        // if if is red: recolor
        // parent => black
        // uncle => black
        // grandparent => red
    }

    fn insert(self: &mut RedBlackTree, key: u32) {
        if let Some(ref mut current_node) = self.root {
            println!("non-empty tree");
            // compare with the tree root node with key
            //recusivly call insertion
            if TreeNode::is_greater(current_node, key) {
                let right_child = &mut current_node.borrow_mut().right;
                if !right_child.root.is_none() {
                    right_child.insert(key);
                } else {
                    println!("inserted left node");
                    let mut new_node = TreeNode::new(key);
                    new_node.parent.root = Some(Rc::clone(current_node));
                    right_child.root = Some(Rc::new(RefCell::new(new_node)));

                    Self::fix(right_child);
                }
            } else {
                let left_child = &mut current_node.borrow_mut().left;
                if !left_child.root.is_none() {
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
    }

    fn print_tree(&self) {
        if let Some(current_node) = &self.root {
            let current_key = current_node.borrow().key;
            let mut current_color = "";

            if current_node.borrow().color == NodeColor::Black {
                current_color = "Black";
            } else {
                current_color = "Red";
            }

            println!("value:{}, color:{}", current_key, current_color);
            if let Some(left_node) = &(*current_node).borrow().left.root {
                let left_key = left_node.borrow().key;
                let mut left_color = "";

                if left_node.borrow().color == NodeColor::Black {
                    left_color = "Black";
                } else {
                    left_color = "Red";
                }
                println!(
                    "{} left node:{} with color {}",
                    current_key, left_key, left_color
                );
                Self::print_tree(&(*current_node).borrow().left);
            } else {
                println!("{} left node is empty", current_key);
            }
            if let Some(right_node) = &(*current_node).borrow().right.root {
                let right_key = right_node.borrow().key;
                let mut right_color = "";

                if right_node.borrow().color == NodeColor::Black {
                    right_color = "Black";
                } else {
                    right_color = "Red";
                }
                println!(
                    "{} right node:{} with color{}",
                    current_key, right_key, right_color
                );
                Self::print_tree(&(*current_node).borrow().right);
            } else {
                println!("{} right node is empty", current_key);
            }
        } else {
            println!("empty node");
        }
    }
}

fn main() {
    /*
    let mut tree: RedBlackTree = RedBlackTree::new();

    println!("do you want to insert?");

    // let line: String = read!("{}\n");
    // println!("{}", line);
    (&mut tree).insert(12);
    println!("tree after inserting 12: {:#?}", tree);
    (&mut tree).insert(1);
    (&mut tree).insert(13);
    (&mut tree).insert(0);

    tree.print_tree();

    // println!("12 1 {:#?}", tree);

    // println!("12 1 13{:#?}", tree);*/

    let mut tree = avl::AVLTree::new();
    tree.insert(10);
    tree.insert(5);

    println!("Tree: {:#?}", tree);
}
