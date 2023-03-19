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

    fn is_red(node: &Rc<RefCell<TreeNode>>) -> bool {
        node.as_ref().borrow().color == NodeColor::Red
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

    // fn print_val(child: &Rc<RefCell<TreeNode>>) {
    //     println!(
    //         "======> fixing child with value : {}",
    //         child.as_ref().borrow().key
    //     );
    // }
    fn print_val_op(child: &Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = child {
            println!("======> child with value : {}", node.as_ref().borrow().key);
        }
    }

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
            gparent.borrow_mut().parent = Self::get_parent(child);
            gparent.borrow_mut().left =
                Self::get_rightchild(Self::get_parent(child).as_ref().unwrap());
        } else {
            println!("RIGHT ROTATION DEBUG:GP IS NULL");
        }
    }
    fn ll_rotation(child: &Rc<RefCell<TreeNode>>, tree: &mut RedBlackTree) {
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
        Self::ll_mutate_grandp(child);
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
        } else {
            println!("RIGHT ROTATION DEBUG:P IS NULL");
        }
    }

    fn lr_p_mutate_child(child: &Rc<RefCell<TreeNode>>, gp: &Rc<RefCell<TreeNode>>) {
        child.as_ref().borrow_mut().left = Self::get_parent(child);

        child.as_ref().borrow_mut().parent = Some(Rc::clone(gp));
    }

    fn lr_rotation(child: &Rc<RefCell<TreeNode>>, tree: &mut RedBlackTree) {
        Self::lr_rotation_p(child);
        Self::ll_rotation(Self::get_leftchild(child).as_ref().unwrap(), tree);
    }
    fn lr_rotation_p(child: &Rc<RefCell<TreeNode>>) {
        let child_rc_1 = Rc::clone(child);
        let parent_rc = Rc::clone(Self::get_grandparent(child).as_ref().unwrap());

        Self::lr_p_mutate_gp(&child_rc_1);
        let child_rc_2 = Rc::clone(child);

        Self::lr_p_mutate_p(&child_rc_2);
        let child_rc_3 = Rc::clone(child);
        Self::lr_p_mutate_child(&child_rc_3, &parent_rc);
    }

// ---------------------------------------- Insert & Insert fix ------------------------------------------
    fn fix_mode(child: &Rc<RefCell<TreeNode>>) -> FixMode {
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

    fn fix(child: &Rc<RefCell<TreeNode>>, tree: &mut RedBlackTree) {
        // Self::print_val(&Rc::clone(child));
        let mode = Self::fix_mode(child);
        match mode {
            FixMode::RotationLeftLeft => {
                println!("          ROTATION LL");

                let p_cpy = Self::get_parent(child);
                let gp_cpy = Self::get_grandparent(child);
                Self::ll_rotation(child, tree);
                Self::swap_color(&p_cpy, &gp_cpy);
                Self::fix(child, tree);
            }
            FixMode::RotationLeftRight => {
                println!("          ROTATION   LEFTRIGHT");

                let c_cpy = Some(Rc::clone(child));
                let gp_cpy = Self::get_grandparent(child);
                Self::lr_rotation(child, tree);

                Self::swap_color(&c_cpy, &gp_cpy);
                Self::fix(child, tree);
            }
            FixMode::RotationRightRight => {
                let p_cpy = Self::get_parent(child);
                let gp_cpy = Self::get_grandparent(child);
                println!("          ROTATION   RR");

                Self::rr_rotation(child, tree);

                Self::swap_color(&p_cpy, &gp_cpy);
                Self::fix(child, tree);
            }
            FixMode::RotationRightLeft => {
                println!("          ROTATION   RIGHTRLEFT");
                let c_cpy = Some(Rc::clone(child));
                let gp_cpy = Self::get_grandparent(child);
                Self::rl_rotation(child, tree);

                Self::swap_color(&c_cpy, &gp_cpy);
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

    fn node_insert(tree: &mut RedBlackTree, key: u32) {
        let ref mut node = tree.root;
        let leaf_node = Self::insert(node, key);
        match leaf_node {
            Some(child) => {
                Self::fix(&child, tree);
            }
            None => {
                eprintln!("new_leaf node return with value: None");
            }
        }
    }

// ---------------------------------------- Get -------------------------------------------
    fn get(node: &Option<Rc<RefCell<TreeNode>>>,key: u32) -> Option<Rc<RefCell<TreeNode>>> {
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
    fn successor(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>{
        let temp = node.as_ref().unwrap().clone();
        if (!temp.as_ref().borrow().left.is_none()) {
            return Self::successor(&temp.as_ref().borrow().left);
        }
        Some(temp)
    }
    fn replace_node(node: &Rc<RefCell<TreeNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        if (node.as_ref().borrow().left.is_some() && node.as_ref().borrow().right.is_some()) {
            return Self:: successor(&node.as_ref().borrow().right);
        }
        if (node.as_ref().borrow().left.is_none() && node.as_ref().borrow().right.is_none()) {
            return None;
        }

        if node.as_ref().borrow().left.is_some() {
            return Some(node.as_ref().borrow().left.as_ref().unwrap().clone())
        } else {
            return Some(node.as_ref().borrow().right.as_ref().unwrap().clone())
        }
        None
    }
        
    fn delete(node: &Option<Rc<RefCell<TreeNode>>>, key: u32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(delete_node) = Self::get(node, key) {
            let u = Self::replace_node(&delete_node);
            
            if Self::is_red(&delete_node) {

            }
            return None
        } else {
            println!("Key not found D:");
            return None;
        }
        None
    }

// ---------------------------------------- Print ------------------------------------------------
fn pretty_print(node: &Option<Rc<RefCell<TreeNode>>>, prefix: &str, is_left: bool) {
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
                if is_left { "┌" } else { "└" },
                color_str);
                println!("{}", node_ref.key);
                let new_prefix = format!("{}{}", prefix, if is_left { "│ " } else { "  " });
                Self::pretty_print(&node_ref.left, &new_prefix, true);
                Self::pretty_print(&node_ref.right, &new_prefix, false);
            }
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

    fn get(&self, key: u32) -> Option<Rc<RefCell<TreeNode>>> {
        TreeNode::get(&self.root, key)
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
    RedBlackTree::tree_insert(&mut tree, 1);
    // println!("          Insert 1");

    // println!("          Insert 2");

    RedBlackTree::tree_insert(&mut tree, 3);

    RedBlackTree::tree_insert(&mut tree, 4);

    RedBlackTree::tree_insert(&mut tree, 5);

    RedBlackTree::tree_insert(&mut tree, 6);

    RedBlackTree::tree_insert(&mut tree, 7);
    RedBlackTree::tree_insert(&mut tree, 0);
    RedBlackTree::tree_insert(&mut tree, 2);

    // RedBlackTree::tree_insert(&mut tree, 15);
    println!("++++++++++++++Pretty tree+++++++++");

    TreeNode::pretty_print(&tree.root, "", false);
    let num = RedBlackTree::count_leaves(&tree);
    println!("number of leaves in the tree:{}", num);
    RedBlackTree::in_order_traversal(&tree);
    tree.get(2);

    // TreeNode::print_tree(&tree.root);
}
