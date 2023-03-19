use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node<T> {
    data: Option<T>,
    height: i32,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}


impl<T> Node<T> {
    fn new(data: Option<T>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            data,
            height: 1,
            left: None,
            right: None,
        }))
    }
}


fn height<T>(node: &Option<Rc<RefCell<Node<T>>>>) -> i32 {
    node.as_ref().map_or(0, |n| n.borrow().height)
}

fn balance_factor<T>(node: &Option<Rc<RefCell<Node<T>>>>) -> i32 {
    // Checks the balance of the nodes by left - right
    height(&node.as_ref().unwrap().borrow().left) - height(&node.as_ref().unwrap().borrow().right)
}

fn update_height<T>(node: &Option<Rc<RefCell<Node<T>>>>) {
    // Check heights of left and right node, take the larger one and add 1.
    let height = std::cmp::max(height(&node.as_ref().unwrap().borrow().left), height(&node.as_ref().unwrap().borrow().right)) + 1;
    node.as_ref().unwrap().borrow_mut().height = height;
}


fn rotate_left<T>(node: Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
    // maps are used in case the value is none on the right side.
    let right = node.borrow().right.as_ref().unwrap().clone();
    let left = right.borrow().left.as_ref().map(|node| node.clone());
    node.borrow_mut().right = left.clone();
    right.borrow_mut().left = Some(node.clone());
    update_height(&right.borrow().left);
    right
}

fn rotate_right<T>(node: Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
    // maps are used in case the value is none on the left side
    let left = node.borrow().left.as_ref().unwrap().clone();
    let right = left.borrow().right.as_ref().map(|node| node.clone());
    node.borrow_mut().left = right.clone();
    left.borrow_mut().right = Some(node.clone());
    update_height(&left.borrow().right);
    left
}
// This rotates the given node left and then right
fn rotate_left_right<T>(node: Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
    let left = node.borrow().left.as_ref().unwrap().clone();
    let new_left = rotate_left(left.clone());
    node.borrow_mut().left = Some(new_left.clone());
    rotate_right(node)
}

// This rotates the given node right and then left
fn rotate_right_left<T>(node: Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
    let right = node.borrow().right.as_ref().unwrap().clone();
    let new_right = rotate_right(right.clone());
    node.borrow_mut().right = Some(new_right.clone());
    rotate_left(node)
}

// https://www.youtube.com/watch?v=vRwi_UcZGjU for explanation on rotation balance logic
fn rebalance<T: Clone>(node: Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
    let balance = balance_factor(&Some(node.clone()));
    if balance > 1 {
        // When tree is right-heavy, perform left rotation then right rotation, otherwise just do a right rotation
        let left_balance = balance_factor(&node.borrow().left);
        if left_balance < 0 {
            rotate_left_right(node)
        } else {
            rotate_right(node)
        }

    } else if balance < -1 {
        // When tree is left-heavy, perform right rotation then left rotation
        let right_balance = balance_factor(&node.borrow().right);
        if right_balance > 0 {
            rotate_right_left(node)
        } else {
            rotate_left(node)
        }
        
    } else {
        node
    }
}

#[derive(Debug)]
pub struct AVLTree<T: std::cmp::Ord> {
    root: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: std::cmp::Ord + Clone + Default + std::fmt::Debug> AVLTree<T> {
    pub fn new() -> Self {
        AVLTree { root: None }
    }

    pub fn insert(&mut self, data: T) {
        let node = Node::new(Some(data));
        self.root = match self.root.take() {
            None => Some(node.clone()),
            Some(root) => Some(Self::insert_recursive(root, node))
        };
    }

    fn insert_recursive(root: Rc<RefCell<Node<T>>>, node: Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
        let root_data = root.borrow().data.clone();
        let node_data = node.borrow().data.clone();
        // Travese left if the data is smaller than the current root
        if node_data < root_data {
             root.replace_with(|old| 
                match old {
                    Node{data, height, left: None, right: Some(y)} => {
                        Node {
                            data: data.clone(),
                            height: height.clone(),
                            left: Some(node.clone()),
                            right: Some(Rc::clone(y)),
                        }
                    },
                    Node{data, height, left: Some(x), right: Some(y)} => {
                        Node {
                            data: data.clone(),
                            height: height.clone(),
                            left: Some(Self::insert_recursive(x.clone(), node.clone())),
                            right: Some(Rc::clone(y)),
                        }
                    },
                    Node{data, height, left: None, right: None} => {
                        Node {
                            data: data.clone(),
                            height: height.clone(),
                            left: Some(node.clone()),
                            right: None,
                        }
                    },
                    Node{data, height, left: Some(x), right: None} => {
                        Node {
                            data: data.clone(),
                            height: height.clone(),
                            left: Some(Self::insert_recursive(x.clone(), node.clone())),
                            right: None,
                        }
                    },
                }
             );  
        // Travese right if the data is larger than the current root
        } else if node_data > root_data {
            root.replace_with(|old| 
                match old {
                    Node{data, height, left: Some(y), right: None} => {
                        Node {
                            data: data.clone(),
                            height: height.clone(),
                            left: Some(Rc::clone(y)),
                            right: Some(node.clone()),
                        }
                    },
                    Node{data, height, left: Some(y), right: Some(x)} => {
                        Node {
                            data: data.clone(),
                            height: height.clone(),
                            left: Some(Rc::clone(y)),
                            right: Some(Self::insert_recursive(x.clone(), node.clone())),
                        }
                    },
                    Node{data, height, left: None, right: None} => {
                        Node {
                            data: data.clone(),
                            height: height.clone(),
                            left: None,
                            right: Some(node.clone()),
                        }
                    },
                    Node{data, height, left: None, right: Some(x)} => {
                        Node {
                            data: data.clone(),
                            height: height.clone(),
                            left: None,
                            right: Some(Self::insert_recursive(x.clone(), node.clone())),
                        }
                    },
                }
            );
        }
        // If the value inserted already exists in the tree then nothing happens
        update_height(&Some(root.clone()));
        rebalance(root)
    }

    pub fn delete(&mut self, data: T) {
        let node = Node::new(Some(data));
        if let Some(root) = self.root.take() {
            let new_root = Self::delete_recursive(Rc::clone(&root), node);
            // Deletes the root if the only value is None
            self.root = if new_root.borrow().data.is_some() {
                Some(new_root)
            } else {
                None
            };
        }
    }

    fn delete_recursive(root: Rc<RefCell<Node<T>>>, node: Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
        let root_data = root.borrow().data.clone();
        let node_data = node.borrow().data.clone();
        // Check which side the deleted node is going to be on
        // Data is smaller than current node, so travers left
        if node_data < root_data {
            root.replace_with(|old| 
                match old {
                    Node{data, height, left: Some(x), right: Some(y)} => {
                        let temp = Self::delete_recursive(x.clone(), node.clone());
                        if temp.borrow().data.clone().is_none(){
                            Node {
                                data: data.clone(),
                                height: height.clone(),
                                left: None,
                                right: Some(Rc::clone(y)),
                            } 
                        } else {
                            Node {
                                data: data.clone(),
                                height: height.clone(),
                                left: Some(temp),
                                right: Some(Rc::clone(y)),
                            } 
                        }
                    },
                    Node{data, height, left: Some(x), right: None} => {
                        let temp = Self::delete_recursive(x.clone(), node.clone());
                        if temp.borrow().data.clone().is_none(){
                            Node {
                                data: data.clone(),
                                height: height.clone(),
                                left: None,
                                right: None,
                            } 
                        } else {
                            Node {
                                data: data.clone(),
                                height: height.clone(),
                                left: Some(temp),
                                right: None,
                            } 
                        }
                    },
                    Node{data, height, left: None, right: None} => {
                        Node {
                            data: data.clone(),
                            height: height.clone(),
                            left: None,
                            right: None,
                        }
                    },
                    Node{data, height, left: None, right: Some(y)} => {
                        Node {
                            data: data.clone(),
                            height: height.clone(),
                            left: None,
                            right: Some(Rc::clone(y)),
                        }
                    },
                }
             ); 
             update_height(&Some(root.clone()));
             rebalance(root.clone())
        // Data is larger than current node so traverse right
        } else if node_data > root_data {
            root.replace_with(|old| 
                match old {
                    Node{data, height, left: Some(x), right: Some(y)} => {
                        let temp = Self::delete_recursive(y.clone(), node.clone());
                        if temp.borrow().data.clone().is_none(){
                            Node {
                                data: data.clone(),
                                height: height.clone(),
                                left: Some(Rc::clone(x)),
                                right: None,
                            } 
                        } else {
                            Node {
                                data: data.clone(),
                                height: height.clone(),
                                left: Some(Rc::clone(x)),
                                right: Some(temp),
                            } 
                        }
                    },
                    Node{data, height, left: None, right: Some(y)} => {
                        let temp = Self::delete_recursive(y.clone(), node.clone());
                        if temp.borrow().data.clone().is_none(){
                            Node {
                                data: data.clone(),
                                height: height.clone(),
                                left: None,
                                right: None,
                            } 
                        } else {
                            Node {
                                data: data.clone(),
                                height: height.clone(),
                                left: None,
                                right: Some(temp),
                            } 
                        }
                    },
                    Node{data, height, left: None, right: None} => {
                        Node {
                            data: data.clone(),
                            height: height.clone(),
                            left: None,
                            right: None,
                        }
                    },
                    Node{data, height, left: Some(x), right: None} => {
                        Node {
                            data: data.clone(),
                            height: height.clone(),
                            left: Some(Rc::clone(x)),
                            right: None,
                        }
                    },
                }
             ); 
             update_height(&Some(root.clone()));
             rebalance(root.clone())

        } else {
            // Node to be deleted found on the current node
            // Now have to replace the node with a new node
            let _ = &root.replace_with(|old| 
                match old {
                    // Case 1: Node has no children
                    // TODO: FIX CASE WHERE THERE IS NO CHILDREN WHEN DELETING
                    Node{data, height, left: None, right: None} => {
                        println!("NO CHILDREN");
                        Node {
                            data: None,
                            height: 0,
                            left: None,
                            right: None,
                        }
                    },
                    // Case 2: Node has both left and right children
                    Node{data, height, left: Some(x), right: Some(y)} => {
                        let min = Self::find_min(Rc::clone(y));
                        if y.borrow().left.clone().is_none() && y.borrow().right.clone().is_none(){
                            let temp = Node {
                                data: min.borrow().data.clone(),
                                height: height.clone(),
                                left: Some(Rc::clone(x)),
                                right: None,
                            }; temp
                        } else {
                            let temp = Node {
                                data: min.borrow().data.clone(),
                                height: height.clone(),
                                left: Some(Rc::clone(x)),
                                right: Some(Rc::clone(y)),
                            };
                            Self::delete_recursive(x.clone(), min);
                            temp
                        }
                    },
                    // Case 3: Node has no left child
                    Node{data, height, left: None, right: Some(y)} => {
                        println!("NO LEFT CHILD");
                        let min = Self::find_min(Rc::clone(y));
                        let result = Node {
                            data: min.borrow().data.clone(),
                            height: height.clone(),
                            left: None,
                            right: None,
                        }; result
                    },
                    // Case 4: Node has no right child
                    Node{data, height, left: Some(y), right: None} => {
                        println!("NO RIGHT CHILD");
                        let min = Self::find_min(Rc::clone(y));
                        let result = Node {
                            data: min.borrow().data.clone(),
                            height: height.clone(),
                            left: None,
                            right: None,
                        }; result
                    }
                }
             );
             update_height(&Some(root.clone()));
             rebalance(root.clone())

        }
    }

    // Will find the min from a given node by continually traversing left
    fn find_min(node: Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
        if let Some(left) = node.borrow().left.clone() {
            return Self::find_min(left);
        }
        node
    }


    // Checks if the root is none, and if it is then the tree is empty
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    // Returns the height of the root
    pub fn get_height(&self) -> i32 {
        self.root.as_ref().map_or(0, |n| n.borrow().height)
    }

    // Counts the number of leaves by checking if a node has no children and uses recursion
    pub fn count_leaves(&self) -> usize {
        fn count_leaves_helper<T: std::cmp::Ord + Clone + Default + std::fmt::Debug>(node: &Option<Rc<RefCell<Node<T>>>>) -> usize {
            match node {
                None => 0,
                Some(n) => {
                    let node_borrow = n.borrow();
                    if node_borrow.left.is_none() && node_borrow.right.is_none() {
                        1
                    } else {
                        count_leaves_helper(&node_borrow.left) + count_leaves_helper(&node_borrow.right)
                    }
                }
            }
        }

        count_leaves_helper(&self.root)
    }

    // This is the function that is called by the main program, then uses the resursive function to get the rest of the nodes in order
    pub fn in_order_traversal(&self){
        if let Some(node) = &self.root {
            let node_borrow = node.borrow();
            Self::in_order_traversal_recursive(node_borrow.left.as_ref());
            println!("{:?}", node_borrow.data);
            Self::in_order_traversal_recursive(node_borrow.right.as_ref());
        }
    }

    // This uses recursion to print out he in order traversal of the AVL tree by traversing through the tree to the left first, then the right
    fn in_order_traversal_recursive(node: Option<&Rc<RefCell<Node<T>>>>) {
        if let Some(n) = node {
            let node_borrow = n.borrow();
            Self::in_order_traversal_recursive(node_borrow.left.as_ref());
            println!("{:?}", node_borrow.data);
            Self::in_order_traversal_recursive(node_borrow.right.as_ref());
        }
    }

    // This will be the fucntion that main calls to print the structure of the tree
    pub fn print_structure(&self) {
        Self::print_node(&self.root, 0, "Root");
    }

    // This uses preorder traversal to print out the tree with structure, where the height of the tree is the widge that is used to space out the strucutre. 
    fn print_node(node: &Option<Rc<RefCell<Node<T>>>>, level: i32, direction: &str) {
        if let Some(n) = node {
            println!("{:>width$} {} {:?}", "", direction, n.borrow().data.clone().unwrap(), width = (level * 4) as usize);
            Self::print_node(&n.borrow().left, level + 1, "Left");
            Self::print_node(&n.borrow().right, level + 1, "Right");
        }
    }

}


fn main(){
	let mut tree = AVLTree::new();
    tree.insert(10);
    tree.insert(5);
    tree.insert(4);
    tree.insert(3);
    tree.insert(2);

    println!("Tree: {:#?}", tree);
}
