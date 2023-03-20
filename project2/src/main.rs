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
enum DoubleBlackFix {
    BlackSibllRotation,
    BlackSiblrRotation,
    BlackSibrrRotation,
    BlackSibrlRotation,
    BlackSibRecolor,
    RedSibLeft,
    RedSibRight,
    Root,
    None,
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
    // ---------------------------------------- Generic Op -------------------------------------------
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

    fn update_height(node: &Rc<RefCell<TreeNode>>) {
        // Check heights of left and right node, take the larger one and add 1.
        let height = std::cmp::max(
            Self::height(&node.as_ref().borrow().left),
            Self::height(&node.as_ref().borrow().right),
        ) + 1;
        node.as_ref().borrow_mut().height = height;
    }

    fn recursive_update_height(node: &Rc<RefCell<TreeNode>>) {
        // Check heights of left and right node, take the larger one and add 1.
        Self::update_height(&Rc::clone(node));
        if let Some(ref parent) = node.as_ref().borrow().parent {
            Self::recursive_update_height(&parent);
        } else {
            // node is the root node
            return;
        }
    }

    fn is_greater(node: &Rc<RefCell<TreeNode>>, z: u32) -> bool {
        if node.as_ref().borrow().key < z {
            return true;
        }
        false
    }

    fn is_equal(node: &Rc<RefCell<TreeNode>>, z: u32) -> bool {
        if node.as_ref().borrow().key == z {
            return true;
        }
        false
    }
    fn get_key(node: &Rc<RefCell<TreeNode>>) -> u32 {
        return node.as_ref().borrow().key;
    }
    fn swap_key(node1: &Rc<RefCell<TreeNode>>, node2: &Rc<RefCell<TreeNode>>) {
        let key1 = Self::get_key(node1);
        let key2 = Self::get_key(node2);
        node1.borrow_mut().key = key2;
        node2.borrow_mut().key = key1;
    }
    fn change_colour(node: &Rc<RefCell<TreeNode>>, color: NodeColor) {
        node.borrow_mut().color = color;
    }
    fn get_color(node: &Rc<RefCell<TreeNode>>) -> NodeColor {
        return node.as_ref().borrow().color.clone();
    }
    fn swap_color(node1: &Option<Rc<RefCell<TreeNode>>>, node2: &Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node_1) = node1 {
            let color1 = Self::get_color(node_1);
            if let Some(node_2) = node2 {
                let color2 = Self::get_color(node_2);
                node_1.as_ref().borrow_mut().color = color2;
                node_2.as_ref().borrow_mut().color = color1;
            }
        }
    }

    fn is_red(node: Rc<RefCell<TreeNode>>) -> bool {
        node.as_ref().borrow().color == NodeColor::Red
    }
    fn is_black(node: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = node {
            node.as_ref().borrow().color == NodeColor::Black
        } else {
            true
        }
    }
    fn is_greater_node(parent: &Rc<RefCell<TreeNode>>, child: &Rc<RefCell<TreeNode>>) -> bool {
        if parent.as_ref().borrow().key < child.as_ref().borrow().key {
            return true;
        }
        false
    }
    // returns false if no parent or is right child
    fn is_left_child(node: &Rc<RefCell<TreeNode>>) -> bool {
        if let Some(parent) = Self::get_parent(node) {
            if let Some(left) = &parent.as_ref().borrow().left {
                return Rc::ptr_eq(&left.clone(), node);
            } else {
                return false;
            };
        } else {
            false
        }
    }

    fn has_only_child(node: &Rc<RefCell<TreeNode>>) -> bool {
        return node.as_ref().borrow().left.is_some() ^ node.as_ref().borrow().right.is_some();
    }

    fn get_sibling(node: &Rc<RefCell<TreeNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(parent) = Self::get_parent(node) {
            if Self::is_left_child(node) {
                if let Some(sibling) = &parent.as_ref().borrow().right {
                    return Some(sibling.clone());
                } else {
                    return None;
                }
            } else {
                if let Some(sibling) = &parent.as_ref().borrow().left {
                    return Some(sibling.clone());
                } else {
                    return None;
                }
            }
        } else {
            None
        }
    }
    fn get_sib_right_child(child: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(child) = child {
            if let Some(sib) = Self::get_sibling(child) {
                if let Some(right_child_sb) = Self::get_rightchild(&sib) {
                    return Some(Rc::clone(&right_child_sb));
                }
            }
        }
        eprintln!("code should not get here: get_sib_right_child");
        return None;
    }

    fn get_sib_left_child(child: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(child) = child {
            if let Some(sib) = Self::get_sibling(child) {
                if let Some(left_child_sb) = Self::get_leftchild(&sib) {
                    return Some(Rc::clone(&left_child_sb));
                }
            }
        }
        eprintln!("code should not get here: get_sib_right_child");
        return None;
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

    fn print_val(child: &Rc<RefCell<TreeNode>>) {
        println!(
            "======> fixing child with value : {}",
            child.as_ref().borrow().key
        );
    }

    // fn print_val_op(child: &Option<Rc<RefCell<TreeNode>>>) {
    //     if let Some(node) = child {
    //         println!("======> child with value : {}", node.as_ref().borrow().key);
    //     }
    // }

    // ---------------------------------------- Rotation Op --------------------------------------
    fn ll_mutate_parent(child: &Rc<RefCell<TreeNode>>, ggp: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(parent) = &child.as_ref().borrow().parent {
            parent.borrow_mut().right = Self::get_grandparent(child);

            parent.borrow_mut().parent = ggp;
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
            println!(
                "LL rotation: grand parent with value:{}",
                gparent.as_ref().borrow().key
            );
            gparent.borrow_mut().parent = Self::get_parent(child);
            gparent.borrow_mut().left =
                Self::get_rightchild(Self::get_parent(child).as_ref().unwrap());
            if let Some(right_child) =
                Self::get_rightchild(Self::get_parent(child).as_ref().unwrap())
            {
                right_child.as_ref().borrow_mut().parent = Some(Rc::clone(gparent));
            }
        } else {
            println!("RIGHT ROTATION DEBUG:GP IS NULL");
        }
    }
    fn ll_rotation(child: &Rc<RefCell<TreeNode>>, tree: &mut RedBlackTree) {
        println!(
            "LL rotation with child value{}",
            child.as_ref().borrow().key
        );
        let ggp_cp = Self::get_greatgrandparent(child);
        if let Some(ggp) = Self::get_greatgrandparent(child) {
            if let Some(gp) = Self::get_grandparent(child) {
                if Self::is_greater(&gp, ggp.as_ref().borrow().key) {
                    ggp.as_ref().borrow_mut().left = Self::get_parent(child);
                } else {
                    ggp.as_ref().borrow_mut().right = Self::get_parent(child);
                }
                println!(
                    "LL rotation gtreat grand parents 's shoud have value{}",
                    ggp.as_ref().borrow().key
                );
            }
        } else {
            tree.root = Self::get_parent(child);
        }
        println!("Tree during LL rotations");
        Self::print_tree(&tree.root);
        println!("===End");

        Self::ll_mutate_grandp(child);
        println!("2. Tree during LL rotations");
        Self::print_tree(&tree.root);
        println!("===End");
        Self::ll_mutate_parent(child, ggp_cp);
        // also need to mutate greategrandparent
    }
    fn rr_mutate_parent(child: &Rc<RefCell<TreeNode>>, ggp: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(parent) = &child.as_ref().borrow().parent {
            parent.borrow_mut().left = Self::get_grandparent(child);

            parent.borrow_mut().parent = ggp;
        } else {
            println!("RIGHT ROTATION DEBUG:P IS NULL");
        }
    }

    fn rr_mutate_grandp(child: &Rc<RefCell<TreeNode>>) {
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
            gparent.borrow_mut().right =
                Self::get_leftchild(Self::get_parent(child).as_ref().unwrap());
            if let Some(left_child) = Self::get_leftchild(Self::get_parent(child).as_ref().unwrap())
            {
                left_child.as_ref().borrow_mut().parent = Self::get_grandparent(child);
            }
        } else {
            println!("RIGHT ROTATION DEBUG:GP IS NULL");
        }
    }

    fn rr_rotation(child: &Rc<RefCell<TreeNode>>, tree: &mut RedBlackTree) {
        let ggp_cp = Self::get_greatgrandparent(child);
        if let Some(ggp) = Self::get_greatgrandparent(child) {
            if let Some(gp) = Self::get_grandparent(child) {
                if Self::is_greater(&gp, ggp.as_ref().borrow().key) {
                    ggp.as_ref().borrow_mut().left = Self::get_parent(child);
                } else {
                    ggp.as_ref().borrow_mut().right = Self::get_parent(child);
                }
            }
        } else {
            tree.root = Self::get_parent(child);
        }
        Self::rr_mutate_grandp(child);
        Self::rr_mutate_parent(child, ggp_cp);
        // also need to mutate greategrandparent
    }

    fn rl_p_mutate_p(child: &Rc<RefCell<TreeNode>>) {
        if let Some(parent) = &child.as_ref().borrow().parent {
            parent.borrow_mut().parent = Some(Rc::clone(child));
            parent.borrow_mut().left = Self::get_rightchild(child);
            if let Some(right_child) = Self::get_rightchild(child) {
                right_child.as_ref().borrow_mut().parent = Some(Rc::clone(parent));
            }
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
    }
    fn rl_rotation(child: &Rc<RefCell<TreeNode>>, tree: &mut RedBlackTree) {
        Self::rl_p_rotation(child);
        Self::rr_rotation(Self::get_rightchild(child).as_ref().unwrap(), tree);
    }
    // lr
    fn lr_p_mutate_gp(child: &Rc<RefCell<TreeNode>>) {
        if let Some(gparent) = &child
            .as_ref()
            .borrow()
            .parent
            .as_ref()
            .unwrap()
            .borrow()
            .parent
        {
            gparent.borrow_mut().left = Some(Rc::clone(child));
        } else {
            println!("RIGHT ROTATION DEBUG:GP IS NULL");
        }
    }
    fn lr_p_mutate_p(child: &Rc<RefCell<TreeNode>>) {
        if let Some(parent) = &child.as_ref().borrow().parent {
            parent.borrow_mut().parent = Some(Rc::clone(child));
            parent.borrow_mut().right = Self::get_leftchild(child);
            if let Some(left_child) = Self::get_leftchild(child) {
                left_child.as_ref().borrow_mut().parent = Self::get_parent(child);
            }
        } else {
            println!("RIGHT ROTATION DEBUG:P IS NULL");
        }
    }

    fn lr_p_mutate_child(child: &Rc<RefCell<TreeNode>>, gp: &Rc<RefCell<TreeNode>>) {
        child.as_ref().borrow_mut().left = Self::get_parent(child);

        child.as_ref().borrow_mut().parent = Some(Rc::clone(gp));
    }

    fn lr_rotation(child: &Rc<RefCell<TreeNode>>, tree: &mut RedBlackTree) {
        println!("+++++++++++++++++++++++++++++++++++beofre lr rotation");
        Self::print_tree(&tree.root);
        TreeNode::pretty_print(&tree.root, "", false, true);

        Self::lr_rotation_p(child);
        println!("+++++++++++++++++++++++++++++++++++middle of lr rotation");
        TreeNode::pretty_print(&tree.root, "", false, true);
        Self::print_tree(&tree.root);
        Self::ll_rotation(Self::get_leftchild(child).as_ref().unwrap(), tree);
    }
    fn lr_rotation_p(child: &Rc<RefCell<TreeNode>>) {
        let child_rc_1 = Rc::clone(child);
        let gparent_rc = Rc::clone(Self::get_grandparent(child).as_ref().unwrap());
        println!(
            "current child's grand pa{}\n child's value : {}",
            gparent_rc.as_ref().borrow().key,
            child.as_ref().borrow().key
        );
        Self::lr_p_mutate_gp(&child_rc_1);
        let child_rc_2 = Rc::clone(child);

        Self::lr_p_mutate_p(&child_rc_2);
        let child_rc_3 = Rc::clone(child);
        Self::lr_p_mutate_child(&child_rc_3, &gparent_rc);
    }

    // ---------------------------------------- Insert & Insert fix ------------------------------------------
    fn fix_mode(child: &Rc<RefCell<TreeNode>>) -> FixMode {
        match child.as_ref().borrow().parent {
            Some(ref parent) => {
                if parent.clone().as_ref().borrow().color == NodeColor::Black {
                    // no fixing needed;
                    // println!("FIX DEBUG: node's parent is black, no fixing needed");
                    return FixMode::None;
                } else {
                    match parent.as_ref().borrow().parent {
                        Some(ref grandp) => {
                            // check uncle.
                            if TreeNode::is_greater(grandp, parent.clone().as_ref().borrow().key) {
                                // parent was the right child
                                // println!("FIX DEBUG: uncle on the left");
                                match grandp.as_ref().borrow().left {
                                    Some(ref uncle) => {
                                        if uncle.clone().as_ref().borrow().color == NodeColor::Red {
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
                                // println!("FIX DEBUG: uncle on the right");

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
                // println!("FIX DEBUG: Child has no parent");
                return FixMode::RecolorRoot;
            }
        }
        return FixMode::None;
    }

    fn fix(child: &Rc<RefCell<TreeNode>>, tree: &mut RedBlackTree) {
        // println!("sdfhkjsdfhfix leaf node:{}", child.as_ref().borrow().key);

        let mode = Self::fix_mode(child);
        println!(
            "after fix modefix leaf node:{}",
            child.as_ref().borrow().key
        );
        match mode {
            FixMode::RotationLeftLeft => {
                println!("          ROTATION LL");
                let child_rc = Rc::clone(&child);
                let p_cpy = Self::get_parent(child);
                let gp_cpy = Self::get_grandparent(child);
                Self::ll_rotation(child, tree);
                Self::swap_color(&p_cpy, &gp_cpy);
                if let Some(gp) = gp_cpy {
                    Self::update_height(&gp);
                }
                Self::recursive_update_height(&child_rc);

                // fix height here
                Self::fix(child, tree);
            }
            FixMode::RotationLeftRight => {
                println!("          ROTATION   LEFTRIGHT");
                println!("CUrrent roation of child:{}", child.as_ref().borrow().key);

                let c_cpy = Some(Rc::clone(child));
                let p_cpy = Self::get_parent(child);

                let gp_cpy = Self::get_grandparent(child);
                Self::lr_rotation(child, tree);

                Self::swap_color(&c_cpy, &gp_cpy);
                if let Some(gp) = gp_cpy {
                    Self::update_height(&gp);
                }
                if let Some(p) = p_cpy {
                    Self::recursive_update_height(&p);
                }

                Self::fix(child, tree);
            }
            FixMode::RotationRightRight => {
                let child_rc = Rc::clone(&child);

                let p_cpy = Self::get_parent(child);
                let gp_cpy = Self::get_grandparent(child);
                println!("          ROTATION   RR");

                Self::rr_rotation(child, tree);

                Self::swap_color(&p_cpy, &gp_cpy);
                if let Some(gp) = gp_cpy {
                    Self::update_height(&gp);
                }
                Self::recursive_update_height(&child_rc);
                Self::fix(child, tree);
            }
            FixMode::RotationRightLeft => {
                println!("          ROTATION   RIGHTRLEFT");
                let c_cpy = Some(Rc::clone(child));
                let p_cpy = Self::get_parent(child);

                let gp_cpy = Self::get_grandparent(child);
                Self::rl_rotation(child, tree);

                Self::swap_color(&c_cpy, &gp_cpy);
                if let Some(gp) = gp_cpy {
                    Self::update_height(&gp);
                }
                if let Some(p) = p_cpy {
                    Self::recursive_update_height(&p);
                }
                Self::fix(child, tree);
            }
            FixMode::RecolorRoot => {
                println!("          RECOLOR:ROOT");
                Self::change_colour(child, NodeColor::Black);
                return;
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
                if let Some(gp) = Self::get_grandparent(child) {
                    Self::fix(&gp, tree);
                } else {
                    eprintln!("Child should have grandp");
                }
                // recursive call on child's grand_parent
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

                // recursive call on child's grand_parent
                if let Some(gp) = Self::get_grandparent(child) {
                    Self::fix(&gp, tree);
                } else {
                    eprintln!("Child should have grandp");
                }
            }
            FixMode::None => {
                println!("          NONE");
                return;
            }
        }
    }

    fn insert(node: &mut Option<Rc<RefCell<TreeNode>>>, key: u32) -> Option<Rc<RefCell<TreeNode>>> {
        let new_leaf: Option<Rc<RefCell<TreeNode>>> = {
            let mut return_leaf: Option<Rc<RefCell<TreeNode>>> = None;
            if let Some(current_node) = node {
                // compare with the tree root node with key
                if TreeNode::is_greater(current_node, key) {
                    let mut t_node = current_node.borrow_mut();
                    if !t_node.right.is_none() {
                        return_leaf = Self::insert(&mut t_node.right, key);
                    } else {
                        // println!("inserted right node");
                        let mut new_node = TreeNode::new(key);
                        new_node.parent = Some(current_node.clone());
                        let new_leaf = Rc::new(RefCell::new(new_node));
                        t_node.right = Some(new_leaf.clone());
                        return_leaf = Some(new_leaf.clone());
                    }
                } else if TreeNode::is_equal(current_node, key) {
                    // duplicated value, do nothing
                    return None;
                } else {
                    let mut t_node = current_node.borrow_mut();
                    if !t_node.left.is_none() {
                        return_leaf = Self::insert(&mut t_node.left, key);
                    } else {
                        // println!("inserted left node");
                        let mut new_node = TreeNode::new(key);
                        new_node.parent = Some(current_node.clone());
                        let new_leaf = Rc::new(RefCell::new(new_node));
                        t_node.left = Some(new_leaf.clone());
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

    pub fn node_insert(tree: &mut RedBlackTree, key: u32) {
        let ref mut node = tree.root;
        let leaf_node = Self::insert(node, key);
        match leaf_node {
            Some(child) => {
                println!("inserted leaf node:{}", child.as_ref().borrow().key);

                Self::fix(&child, tree);
                if key.clone() == 40 {
                    println!("fix double black tree! on key{}", key.clone());
                    TreeNode::fix_double_black(&Self::get_parent(&child), tree);
                }
            }
            None => {
                println!("This key already exist");
            }
        }
    }

    // ---------------------------------------- Get -------------------------------------------
    fn get(node: &Option<Rc<RefCell<TreeNode>>>, key: u32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(current_node) = node {
            if current_node.as_ref().borrow().key == key {
                // println!("{:?}", current_node.as_ref().borrow().key);
                return Some(current_node.clone());
            } else if current_node.as_ref().borrow().key > key {
                return Self::get(&current_node.as_ref().borrow().left, key);
            } else {
                return Self::get(&current_node.as_ref().borrow().right, key);
            }
        } else {
            return None;
        }
    }

    // ---------------------------------------- Delete ------------------------------------------
    fn successor(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let temp = node.as_ref().unwrap().clone();
        if (!temp.as_ref().borrow().left.is_none()) {
            return Self::successor(&temp.as_ref().borrow().left);
        }
        Some(temp)
    }
    fn replace_node(node: &Rc<RefCell<TreeNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        if node.as_ref().borrow().left.is_some() && node.as_ref().borrow().right.is_some() {
            return Self::successor(&node.as_ref().borrow().right);
        }
        if node.as_ref().borrow().left.is_none() && node.as_ref().borrow().right.is_none() {
            return None;
        }

        if node.as_ref().borrow().left.is_some() {
            return Some(node.as_ref().borrow().left.as_ref().unwrap().clone());
        } else {
            return Some(node.as_ref().borrow().right.as_ref().unwrap().clone());
        }
    }

    fn delete(delete_node: Rc<RefCell<TreeNode>>, tree: &mut RedBlackTree, root_key: u32) {
        let u = Self::replace_node(&delete_node);
        // let uvBlack = (u.is_none() || Self::is_black(u));
        let parent = Self::get_parent(&delete_node);

        // // Debugging print
        // if let Some(p) = &parent {
        //     Self::print_val(&delete_node.as_ref().borrow().parent.as_ref().unwrap());
        //     println!("******************************************parent db************************");
        //     Self::print_val(p);
        // }

        if u.is_none() {
            if Self::is_equal(&delete_node, root_key) {
                tree.root = None;
            } else {
                if !Self::is_red(Rc::clone(&delete_node)) {
                    Self::fix_double_black(&Some(Rc::clone(&delete_node)), tree);
                } else {
                    // sibling is not null, make it red"  *** Does not make sense to change sibling here
                }

                if Self::is_left_child(&delete_node) {
                    if let Some(parent) = Self::get_parent(&delete_node) {
                        parent.borrow_mut().left = None;
                    };
                } else {
                    if let Some(parent) = Self::get_parent(&delete_node) {
                        parent.borrow_mut().right = None;
                    };
                }
            }
            return;
        }

        if Self::has_only_child(&delete_node) {
            if Self::is_equal(&delete_node, root_key) {
                tree.root = u.clone();
                if let Some(replace_node) = &u {
                    let mut temp = replace_node.borrow_mut();
                    temp.parent = None;
                    temp.color = NodeColor::Black;
                }
            } else {
                if Self::is_left_child(&delete_node) {
                    if let Some(parent) = Self::get_parent(&delete_node) {
                        if let Some(current_node) = &u {
                            parent.borrow_mut().left = Some(current_node.clone());
                            current_node.borrow_mut().parent = Some(parent.clone());
                        }
                    }
                } else {
                    if let Some(parent) = Self::get_parent(&delete_node) {
                        if let Some(current_node) = &u {
                            parent.borrow_mut().right = Some(current_node.clone());
                            current_node.borrow_mut().parent = Some(parent.clone());
                        }
                    }
                }

                if Self::is_black(&u) && Self::is_black(&Some(delete_node.clone())) {
                    if let Some(current_node) = &u {
                        Self::fix_double_black(&Some(current_node.clone()), tree);
                    }
                } else {
                    if let Some(current_node) = &u {
                        current_node.borrow_mut().color = NodeColor::Black;
                    }
                }
            }
            return;
        }
        if let Some(current_node) = &u {
            // if let Some(temp) = Self::get_parent(current_node) {
            //     Self::print_val(&temp);
            // }
            // if let Some(temp) = Self::get_parent(&delete_node) {
            //     Self::print_val(&temp);
            // }
            Self::swap_key(&delete_node, current_node);
            // println!("*********************delete node{}", Self::get_key(&delete_node));
            // println!("*********************current node{}", Self::get_key(&current_node));
            // if let Some(temp) = Self::get_parent(current_node) {
            //     Self::print_val(&temp);
            // }
            Self::delete(current_node.clone(), tree, root_key)
        }
    }
    fn fix_double_black_helper(node: &Option<Rc<RefCell<TreeNode>>>) -> DoubleBlackFix {
        println!("Double black fix helper");
        if let Some(child) = node {
            if let Some(sib) = Self::get_sibling(child) {
                if sib.as_ref().borrow().color == NodeColor::Red {
                    //rotation case
                    if Self::is_left_child(&sib) {
                        return DoubleBlackFix::RedSibLeft;
                    } else {
                        return DoubleBlackFix::RedSibRight;
                    }
                } else {
                    println!("Sib is black");
                    // sib is black
                    if Self::is_black(&Self::get_leftchild(&sib))
                        && Self::is_black(&Self::get_rightchild(&sib))
                    {
                        // recolor
                        // 2 black children

                        return DoubleBlackFix::BlackSibRecolor;
                    } else {
                        if Self::is_left_child(&sib) {
                            if !Self::is_black(&Self::get_leftchild(&sib)) {
                                return DoubleBlackFix::BlackSibllRotation;
                            } else {
                                return DoubleBlackFix::BlackSiblrRotation;
                            }
                        } else {
                            if !Self::is_black(&Self::get_leftchild(&sib)) {
                                return DoubleBlackFix::BlackSibrlRotation;
                            } else {
                                return DoubleBlackFix::BlackSibrrRotation;
                            }
                        }
                    }
                }
            } else {
                // sib is black
                println!("sib is null");
                return DoubleBlackFix::None;
            }
        } else {
            println!("current node is nul");
            return DoubleBlackFix::Root;
        }
    }

    fn fix_double_black(u: &Option<Rc<RefCell<TreeNode>>>, tree: &mut RedBlackTree) {
        match Self::fix_double_black_helper(u) {
            DoubleBlackFix::BlackSibllRotation => {
                println!("=====>BalckSibllRotation");
            }
            DoubleBlackFix::BlackSiblrRotation => {
                println!("=====>BlackSiblrRotation");
            }
            DoubleBlackFix::BlackSibrrRotation => {
                println!("=====>BlackSibrrRotation");
            }
            DoubleBlackFix::BlackSibrlRotation => {
                println!("=====>BlackSibrlRotation");
            }
            DoubleBlackFix::BlackSibRecolor => {
                println!("=====>BlackSibRecolor");
                if let Some(child) = u {
                    if let Some(sib) = Self::get_sibling(child) {
                        sib.as_ref().borrow_mut().color == NodeColor::Red;
                        if Self::is_black(&Self::get_parent(child)) {
                            Self::fix_double_black(&Self::get_parent(child), tree);
                        } else {
                            Self::get_parent(child).as_ref().unwrap().borrow_mut().color =
                                NodeColor::Black;
                        }
                    }
                }
            }
            DoubleBlackFix::RedSibLeft => {}
            DoubleBlackFix::RedSibRight => {

                // Self::rr_rotation(child, tree);
            }
            DoubleBlackFix::Root => {}
            DoubleBlackFix::None => {}
        }
    }

    // ---------------------------------------- Print ------------------------------------------------
    fn pretty_print(
        node: &Option<Rc<RefCell<TreeNode>>>,
        prefix: &str,
        is_left: bool,
        is_root: bool,
    ) {
        if is_root {
            println!("++++++++++++++Pretty tree+++++++++");
            println!(" ┌ denotes left, └ denotes right\n");
        }
        match node {
            None => return,
            Some(n) => {
                let node_ref = n.borrow();
                let color_str = match node_ref.color {
                    NodeColor::Red => "R",
                    NodeColor::Black => "B",
                };
                print!(
                    "{}{}{}─",
                    prefix,
                    if is_left {
                        "┌-"
                    } else if is_root {
                        "- "
                    } else {
                        "└-"
                    },
                    color_str
                );
                println!("{}", node_ref.key);
                let new_prefix = format!("{}{}", prefix, if is_left { "│ " } else { "  " });
                Self::pretty_print(&node_ref.left, &new_prefix, true, false);
                Self::pretty_print(&node_ref.right, &new_prefix, false, false);
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
            if let Some(p) = Self::get_parent(&current_node) {
                println!(
                    "value:{}, color:{}, height:{},parent{}",
                    current_key,
                    current_color,
                    current_node.as_ref().borrow().height,
                    p.as_ref().borrow().key
                );
            } else {
                println!(
                    "ROOT: value:{}, color:{}, height:{}",
                    current_key,
                    current_color,
                    current_node.as_ref().borrow().height
                );
            }

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

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
    pub fn count_leaves(&self) -> usize {
        fn count_leaves_helper(node: &Option<Rc<RefCell<TreeNode>>>) -> usize {
            match node {
                None => 0,
                Some(n) => {
                    let node_borrow = n.borrow();
                    if node_borrow.left.is_none() && node_borrow.right.is_none() {
                        1
                    } else {
                        count_leaves_helper(&node_borrow.left)
                            + count_leaves_helper(&node_borrow.right)
                    }
                }
            }
        }

        count_leaves_helper(&self.root)
    }

    pub fn tree_insert(&mut self, key: u32) {
        if let Some(ref mut current_node) = self.root {
            // have a node already
            println!("insert another node with key:{}", key.clone());

            TreeNode::node_insert(self, key);
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

    pub fn in_order_traversal(&self) {
        if let Some(node) = &self.root {
            let node_borrow = node.borrow();
            Self::in_order_traversal_recursive(node_borrow.left.as_ref());
            println!("{:?}", node_borrow.key);
            Self::in_order_traversal_recursive(node_borrow.right.as_ref());
        }
    }

    // Uses recursion to print out he in order traversal of the AVL tree by traversing through the tree to the left first, then the right
    fn in_order_traversal_recursive(node: Option<&Rc<RefCell<TreeNode>>>) {
        if let Some(n) = node {
            let node_borrow = n.borrow();
            Self::in_order_traversal_recursive(node_borrow.left.as_ref());
            println!("{:?}", node_borrow.key);
            Self::in_order_traversal_recursive(node_borrow.right.as_ref());
        }
    }

    pub fn get(&self, key: u32) -> Option<Rc<RefCell<TreeNode>>> {
        TreeNode::get(&self.root, key)
    }

    pub fn root_key(&self) -> Option<u32> {
        if let Some(root) = &self.root {
            return Some(root.as_ref().borrow().key);
        } else {
            None
        }
    }
    pub fn delete(&mut self, key: u32) {
        if let Some(root) = &self.root {
            if let Some(delete_node) = TreeNode::get(&self.root, key) {
                let root_key = self.root_key().unwrap();
                TreeNode::delete(delete_node, self, root_key);
            } else {
                println!("Node not found D:");
            }
        } else {
            println!("Empty Tree D:");
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
    // 32 40 20 19 15 30 31 21 38 39 41

    let mut tree = RedBlackTree::new();
    RedBlackTree::tree_insert(&mut tree, 32);
    RedBlackTree::tree_insert(&mut tree, 41);
    RedBlackTree::tree_insert(&mut tree, 20);
    RedBlackTree::tree_insert(&mut tree, 19);
    RedBlackTree::tree_insert(&mut tree, 15);
    RedBlackTree::tree_insert(&mut tree, 30);
    RedBlackTree::tree_insert(&mut tree, 31);
    RedBlackTree::tree_insert(&mut tree, 21);
    // RedBlackTree::tree_insert(&mut tree, 37);
    // RedBlackTree::tree_insert(&mut tree, 39);

    // RedBlackTree::tree_insert(&mut tree, 40);

    // RedBlackTree::tree_insert(&mut tree, 15);

    // RedBlackTree::tree_insert(&mut tree, 15);

    // RedBlackTree::tree_insert(&mut tree, 15);
    TreeNode::pretty_print(&tree.root, "", false, true);
    tree.delete(20);
    TreeNode::pretty_print(&tree.root, "", false, true);
    let num = RedBlackTree::count_leaves(&tree);
    println!("number of leaves in the tree:{}", num);
    RedBlackTree::in_order_traversal(&tree);
    tree.get(2);
    println!("tree with height:{}", TreeNode::height(&tree.root));

    TreeNode::print_tree(&tree.root);
    // RedBlackTree::tree_insert(&mut tree, 37);
    // RedBlackTree::tree_insert(&mut tree, 39);

    // RedBlackTree::tree_insert(&mut tree, 40);
}
