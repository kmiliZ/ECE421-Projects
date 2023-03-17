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

impl<T: std::cmp::Ord + Clone> AVLTree<T> {
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
