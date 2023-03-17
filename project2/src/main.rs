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
    fn fix(self: &mut RedBlackTree) {
        match &self.root {
            Some(current_node) => {
                current_node.replace_with(|old| match old {
                    TreeNode {
                        color,
                        key,
                        parent: RedBlackTree { root: None },
                        left: l,
                        right: r,
                    }// if it current node does not have a parent, it is the root => recolor to black
                     => TreeNode {
                        color: NodeColor::Black,
                        key: old.key.clone(),
                        parent: RedBlackTree { root: None },
                        left: l.clone(),
                        right: r.clone(),
                    },
                    TreeNode {
                        color,
                        key,
                        parent:
                            RedBlackTree {
                                root: Some(par_node),
                            },
                        left,
                        right,
                    } => par_node.replace_with(|par| match par {
                        // if it has a parent, then we match parent's color
                        TreeNode {
                            color: NodeColor::Black,
                            key,
                            parent: p,
                            left: l,
                            right: r,
                        } => {
                            // black parent, nothing to fix
                            TreeNode {
                                color: NodeColor::Black,
                                key: par.key.clone(),
                                parent: p.clone(),
                                left: l.clone(),
                                right: r.clone(),
                            }
                        }
                        TreeNode {
                            // red parent, fix depends on uncle's color
                            color: NodeColor::Red,
                            key,
                            parent: gpar,
                            left,
                            right,
                        } => match par {
                            TreeNode {
                                color,
                                key,
                                parent: RedBlackTree { root: None },
                                left: l,
                                right: r,
                            }//parent is the root a.k.a parent.parent = nll node
                             => {TreeNode {
                                color: NodeColor::Black,
                                key: par.key.clone(),
                                parent: RedBlackTree { root: None },
                                left: l.clone(),
                                right: r.clone(),
                            }},
                            TreeNode {
                                color,
                                key,
                                parent: RedBlackTree { root: Some(gp) },
                                left: l,
                                right: r,
                            } => {
                                gp.replace_with(|gp| {
                                    if gp.key<par.key{
                                        // parent was on the right of gp== uncle on the left
                                        match gp {
                                            TreeNode {
                                                color,
                                                key,
                                                parent: p,
                                                left: RedBlackTree{root:None},
                                                right: r,
                                            } => {
                                                // uncle is nll == uncle is black
                                                // rotate
                                            }
                                            TreeNode {
                                                color,
                                                key,
                                                parent: p,
                                                left: RedBlackTree{root:Some(l)},
                                                right: r,
                                            } => {
                                                if Rc::clone(l).borrow().color == NodeColor::Red {
                                                   // recolor 
                                                } else {
                                                    // rotate
                                                }
                                            }
                                        }
                                    } else {
                                        // parent on the left of gp == uncle on the right
                                        match gp {
                                            TreeNode {
                                                color,
                                                key,
                                                parent: p,
                                                left: l,
                                                right: RedBlackTree{root:None},
                                            } => {
                                                // uncle is nll == uncle is black
                                                // rotate
                                            }
                                            TreeNode {
                                                color,
                                                key,
                                                parent: p,
                                                left: l,
                                                right: RedBlackTree{root:Some(r)},
                                            } => {
                                                if Rc::clone(r).borrow().color == NodeColor::Red {
                                                   // uncle is red =>recolor 
                                                } else {
                                                    // rotate
                                                }
                                            }
                                        }

                                    }
                                    return TreeNode::new(2);
                                }
                                )
                            },
                        },
                    }),
                });
            }
            None => todo!(),
        }
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

    // let mut tree = avl::AVLTree::new();
    // tree.insert(10);
    // tree.insert(5);

    // println!("Tree: {:#?}", tree);
}
