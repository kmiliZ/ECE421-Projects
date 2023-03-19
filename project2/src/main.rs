use text_io::read;

mod avl;
use std::cell::RefCell;
use std::env;
use std::process::Child;
use std::rc::Rc;
#[derive(Clone, Debug, PartialEq)]

enum NodeColor {
    Red,
    Black,
}

enum FixMode {
    RotationLeftLeft,
    RotationLeftRight,
    RotationRightRight,
    RotationRightLeft,
    RecolorRoot,
    RecolorUncleRight,
    RecolorUncleLeft,
    None,
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

    fn change_colour(node: &Rc<RefCell<TreeNode>>, color: NodeColor) {
        node.borrow_mut().color = color;
    }
    fn is_greater(node: &Rc<RefCell<TreeNode>>, z: u32) -> bool {
        if node.as_ref().borrow().key < z {
            return true;
        }
        false
    }
    fn fix_mode(child: &Rc<RefCell<TreeNode>>) -> FixMode {
        println!("Fixing");
        match child.as_ref().borrow().parent {
            Some(ref parent) => {
                if parent.clone().as_ref().borrow().color == NodeColor::Black {
                    // no fixing needed;
                    println!("FIX DEBUG: node's parent is black, no fixing needed");
                    return FixMode::None;
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
                                            // Self::fix(grandp);
                                            return FixMode::RecolorUncleLeft;
                                        } else {
                                            // rotate
                                            if TreeNode::is_greater(
                                                parent,
                                                child.clone().as_ref().borrow().key,
                                            ) {
                                                return FixMode::RotationRightRight;
                                            } else {
                                                return FixMode::RotationRightLeft;
                                            }
                                        }
                                    }
                                    None => {
                                        // rotate
                                        if TreeNode::is_greater(
                                            parent,
                                            child.clone().as_ref().borrow().key,
                                        ) {
                                            return FixMode::RotationRightRight;
                                        } else {
                                            return FixMode::RotationRightLeft;
                                        }
                                    }
                                }
                            } else {
                                println!("FIX DEBUG: uncle on the right");

                                match grandp.as_ref().borrow().right {
                                    Some(ref uncle) => {
                                        if uncle.clone().as_ref().borrow().color == NodeColor::Red {
                                            // recolor;
                                            return FixMode::RecolorUncleRight;
                                        } else {
                                            // rotate
                                            if TreeNode::is_greater(
                                                parent,
                                                child.clone().as_ref().borrow().key,
                                            ) {
                                                return FixMode::RotationLeftRight;
                                            } else {
                                                return FixMode::RotationLeftLeft;
                                            }
                                        }
                                    }
                                    None => {
                                        // rotate
                                        if TreeNode::is_greater(
                                            parent,
                                            child.clone().as_ref().borrow().key,
                                        ) {
                                            return FixMode::RotationLeftRight;
                                        } else {
                                            return FixMode::RotationLeftLeft;
                                        }
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
                return FixMode::RecolorRoot;
            }
        }
        return FixMode::None;
    }
    fn is_greater_node(parent: &Rc<RefCell<TreeNode>>, child: &Rc<RefCell<TreeNode>>) -> bool {
        if parent.as_ref().borrow().key < child.as_ref().borrow().key {
            return true;
        }
        false
    }
    fn get_parent(child: &Rc<RefCell<TreeNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(parent) = &child.as_ref().borrow().parent {
            return Some(Rc::clone(parent));
        } else {
            return None;
        }
    }

    fn get_grandparent(child: &Rc<RefCell<TreeNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(gparent) = &child
            .as_ref()
            .borrow()
            .parent
            .as_ref()
            .unwrap()
            .borrow()
            .parent
        {
            return Some(Rc::clone(gparent));
        } else {
            return None;
        }
    }
    fn get_greatgrandparent(child: &Rc<RefCell<TreeNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref ggparent) = Self::get_grandparent(child)
            .as_ref()
            .unwrap()
            .borrow()
            .parent
        {
            return Some(Rc::clone(&ggparent));
        } else {
            return None;
        }
    }

    fn get_leftchild(child: &Rc<RefCell<TreeNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(l) = &child.as_ref().borrow().left {
            return Some(Rc::clone(l));
        } else {
            return None;
        }
    }

    fn get_rightchild(child: &Rc<RefCell<TreeNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = &child.as_ref().borrow().right {
            return Some(Rc::clone(r));
        } else {
            return None;
        }
    }

    fn ll_mutate_parent(child: &Rc<RefCell<TreeNode>>) {
        if let Some(parent) = &child.as_ref().borrow().parent {
            parent.borrow_mut().parent = Self::get_greatgrandparent(child);
            parent.borrow_mut().right = Self::get_grandparent(child);
        } else {
            println!("RIGHT ROTATION DEBUG:P IS NULL");
        }
    }

    fn ll_mutate_grandp(child: &Rc<RefCell<TreeNode>>) {
        if let Some(gparent) = &child
            .as_ref()
            .borrow()
            .parent
            .as_ref()
            .unwrap()
            .borrow()
            .parent
        {
            gparent.borrow_mut().parent = Self::get_parent(child);
            gparent.borrow_mut().left =
                Self::get_rightchild(Self::get_parent(child).as_ref().unwrap());
        } else {
            println!("RIGHT ROTATION DEBUG:GP IS NULL");
        }
    }
    fn ll_rotation(child: &Rc<RefCell<TreeNode>>) {
        Self::ll_mutate_parent(child);
        Self::ll_mutate_grandp(child);
    }
    fn rl_p_mutate_p(child: &Rc<RefCell<TreeNode>>) {
        if let Some(parent) = &child.as_ref().borrow().parent {
            parent.borrow_mut().parent = Some(Rc::clone(child));
            parent.borrow_mut().left = Self::get_rightchild(child);
        } else {
            println!("RIGHT ROTATION DEBUG:P IS NULL");
        }
    }

    fn rl_p_mutate_gp(child: &Rc<RefCell<TreeNode>>) {
        if let Some(gparent) = &child
            .as_ref()
            .borrow()
            .parent
            .as_ref()
            .unwrap()
            .borrow()
            .parent
        {
            gparent.borrow_mut().right = Some(Rc::clone(child));
        } else {
            println!("RIGHT ROTATION DEBUG:GP IS NULL");
        }
    }

    fn rl_p_mutate_child(child: &Rc<RefCell<TreeNode>>, gp: &Rc<RefCell<TreeNode>>) {
        child.as_ref().borrow_mut().right = Self::get_parent(child);

        child.as_ref().borrow_mut().parent = Some(Rc::clone(gp));
    }

    fn rl_p_rotation(child: &Rc<RefCell<TreeNode>>) {
        let child_rc_1 = Rc::clone(child);
        let parent_rc = Rc::clone(Self::get_grandparent(child).as_ref().unwrap());

        Self::rl_p_mutate_gp(&child_rc_1);
        let child_rc_2 = Rc::clone(child);

        Self::rl_p_mutate_p(&child_rc_2);
        let child_rc_3 = Rc::clone(child);
        Self::rl_p_mutate_child(&child_rc_3, &parent_rc);

        println!("right rotation");
    }

    fn fix(child: &Rc<RefCell<TreeNode>>) {
        let mode = Self::fix_mode(child);
        match mode {
            FixMode::RotationLeftLeft => {
                println!("          ROTATIONLEFTLEFT");
                Self::ll_rotation(child);
            }
            FixMode::RotationLeftRight => {
                println!("          ROTATION   LEFTRIGHT");
                // Self::rl_p_rotation(child);
            }
            FixMode::RotationRightRight => {
                println!("          ROTATION   RIGHTRIGHT");
            }
            FixMode::RotationRightLeft => {
                println!("          ROTATION   RIGHTRLEFT");
                // clone child,child's left parent, grandp 's RC pointer
                // function(RC(child),RC(parent)) change child' right to parent, chanhe parent's parent to child. gp's child to child
                Self::rl_p_rotation(child);
            }
            FixMode::RecolorRoot => {
                println!("          RECOLOR:ROOT");
                Self::change_colour(child, NodeColor::Black);
            }
            FixMode::RecolorUncleRight => {
                println!("          RECOLORRight");
                if let Some(parent) = &child.as_ref().borrow().parent {
                    Self::change_colour(parent, NodeColor::Black);
                    if let Some(grand_parent) = &parent.as_ref().borrow().parent {
                        Self::change_colour(grand_parent, NodeColor::Red);
                        if let Some(uncle) = &grand_parent.as_ref().borrow().right {
                            Self::change_colour(uncle, NodeColor::Black);
                        };
                    };
                };
            }
            FixMode::RecolorUncleLeft => {
                println!("          RECOLORLeft");
                if let Some(parent) = &child.as_ref().borrow().parent {
                    Self::change_colour(parent, NodeColor::Black);
                    if let Some(grand_parent) = &parent.as_ref().borrow().parent {
                        Self::change_colour(grand_parent, NodeColor::Red);
                        if let Some(uncle) = &grand_parent.as_ref().borrow().left {
                            Self::change_colour(uncle, NodeColor::Black);
                        };
                    };
                };
            }
            FixMode::None => {
                println!("          NONE");
            }
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
            println!("insert another node with key:{}", key.clone());

            TreeNode::node_insert(&mut self.root, key);
        } else {
            // case x is the root
            println!("insert root node!");
            let mut new_node = TreeNode::new(key);
            new_node.color = NodeColor::Black;
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
    env::set_var("RUST_BACKTRACE", "1");

    // println!("do you want to insert?");

    // // let line: String = read!("{}\n");
    // // println!("{}", line);

    let mut tree = RedBlackTree::new();
    println!("          Insert 13");
    RedBlackTree::tree_insert(&mut tree, 13);
    println!("          Insert 1");

    println!("          Insert 2");

    RedBlackTree::tree_insert(&mut tree, 15);
    RedBlackTree::tree_insert(&mut tree, 14);

    TreeNode::print_tree(&tree.root);
}
