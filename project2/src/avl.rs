use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node<T> {
    data: T,
    height: i32,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Rc<RefCell<Self>> {
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

// https://www.youtube.com/watch?v=vRwi_UcZGjU for explanation on rotation balance logic
fn rebalance<T>(node: Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
    let balance = balance_factor(&Some(node.clone()));
    if balance > 1 {
        // When tree is right-heavy, perform left rotation then right rotation, otherwise just do a right rotation
        
        let left_balance = balance_factor(&node.borrow().left);
        if left_balance < 0 {
            node.borrow_mut().left = Some(rotate_left(node.borrow().left.as_ref().unwrap().clone()));
        }
        rotate_right(node)
    } else if balance < -1 {
        // When tree is left-heavy, perform right rotation then left rotation
        let right_balance = balance_factor(&node.borrow().right);
        if right_balance > 0 {
            node.borrow_mut().right = Some(rotate_right(node.borrow().right.as_ref().unwrap().clone()));
        }
        rotate_left(node)
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
        let node = Node::new(data);
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
        } else {
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
        update_height(&Some(root.clone()));
        rebalance(root)
    }

    pub fn delete(&mut self, data: T) {
        let node = Node::new(data);
        self.root = match self.root.take() {
            None => Some(node.clone()),
            Some(root) => Some(Self::delete_recursive(root, node))
        };
    }

    fn delete_recursive(root: Rc<RefCell<Node<T>>>, node: Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
        let root_data = root.borrow().data.clone();
        let node_data = node.borrow().data.clone();
        // Check which side the deleted node is going to be on
        if node_data < root_data {
            root.replace_with(|old| 
                match old {
                    Node{data, height, left: Some(x), right: Some(y)} => {
                        Node {
                            data: data.clone(),
                            height: height.clone(),
                            left: Some(Self::delete_recursive(x.clone(), node.clone())),
                            right: Some(Rc::clone(y)),
                        }
                    },
                    Node{data, height, left: Some(x), right: None} => {
                        Node {
                            data: data.clone(),
                            height: height.clone(),
                            left: Some(Self::delete_recursive(x.clone(), node.clone())),
                            right: None,
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
             update_height(&Some(node.clone()));
             rebalance(node.clone())
             
        } else if node_data > root_data {
            root.replace_with(|old| 
                match old {
                    Node{data, height, left: Some(x), right: Some(y)} => {
                        Node {
                            data: data.clone(),
                            height: height.clone(),
                            left: Some(Rc::clone(x)),
                            right: Some(Self::delete_recursive(y.clone(), node.clone())),
                        }
                    },
                    Node{data, height, left: None, right: Some(y)} => {
                        Node {
                            data: data.clone(),
                            height: height.clone(),
                            left: None,
                            right: Some(Self::delete_recursive(y.clone(), node.clone())),
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
            /*
            // Node to be deleted found on the current node
            // Now have to replace the node with a new node
            if node.borrow().left.is_none() && node.borrow().right.is_none(){
                // TODO: Case 1: Node has no children

            } else if node.borrow().left.is_none() {
                // Case 2: Node has no left child
                let right_child = node.borrow_mut().right.take();
                node = right_child.unwrap().clone();

            } else if node.borrow().right.is_none() {
                // Case 3: Node has no right child
                let left_child = node.borrow_mut().left.take();
                node = left_child.unwrap().clone();

            } else {
                // Case 4: Node has both left and right children
                // Find the smallest child on the right side and replace the deleted node with that
                let mut min_node = Rc::clone(&node.borrow().right.as_ref().unwrap());
                let mut left_child = min_node.borrow().left.clone();
                while min_node.borrow().left.is_some() {
                    min_node = Rc::clone(&left_child.unwrap());
                    left_child = min_node.borrow().left.clone();
                }
                // Replace the node to be deleted with the minimum node found
                let new_data = min_node.borrow().data.clone();
                Self::delete_recursive(node, new_data.clone());
                node.borrow_mut().data = new_data;
            }
            update_height(&Some(node.clone()));
            rebalance(node.clone())*/
            root.replace_with(|old| 
                match old {
                    // TODO: FIX CASE WHERE THERE IS NO CHILDREN WHEN DELETING
                    Node{data, height, left: None, right: None} => {
                        Node {
                            data: T::default(),
                            height: 0,
                            left: None,
                            right: None,
                        }
                    },
                    Node{data, height, left: Some(x), right: Some(y)} => {
                        let min = Self::find_min(Rc::clone(y));
                        let min2 = Self::find_min(Rc::clone(y));
                        let new_right = Self::delete_recursive(Rc::clone(y), min2);
                        let x = Node {
                            data: min.borrow().data.clone(),
                            height: height.clone(),
                            left: Some(Rc::clone(x)),
                            right: Some(new_right),
                        }; x
                    },
                    Node{data, height, left: None, right: Some(y)} => {
                        let min = Self::find_min(Rc::clone(y));
                        let x = Node {
                            data: min.borrow().data.clone(),
                            height: height.clone(),
                            left: None,
                            right: None,
                        }; x
                    },
                    Node{data, height, left: Some(y), right: None} => {
                        let min = Self::find_min(Rc::clone(y));
                        let x = Node {
                            data: min.borrow().data.clone(),
                            height: height.clone(),
                            left: None,
                            right: None,
                        }; x
                    }
                }
             );
             update_height(&Some(root.clone()));
             rebalance(root.clone())

            /*
            match (&root.borrow().left, &root.borrow().right) {
                (None, Some(x)) => Some(Rc::clone(x)),
                (Some(x), None) => Some(Rc::clone(x)),
                (Some(x), Some(y)) => {
                    let min = Self::find_min(Rc::clone(y));
                    let new_right = Self::delete_recursive(Rc::clone(y), min);
                    let mut new_root = Node {
                        data: min.borrow().data.clone(),
                        height: root.borrow().height.clone(),
                        left: Some(Rc::clone(x)),
                        right: Some(new_right),
                    };
                    update_height(&Some(new_root));
                    rebalance(new_root)
                }
            }*/
        }
    }

    fn find_min(node: Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
        if let Some(left) = node.borrow().left.clone() {
            return Self::find_min(left);
        }
        node
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
