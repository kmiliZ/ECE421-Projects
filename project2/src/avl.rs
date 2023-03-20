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

// Gets the height for the update_height function
fn height<T>(node: &Option<Rc<RefCell<Node<T>>>>) -> i32 {
    node.as_ref().map_or(0, |n| n.borrow().height)
}

// Checks the balance of the nodes by left - right
fn balance_factor<T>(node: &Option<Rc<RefCell<Node<T>>>>) -> i32 {
    height(&node.as_ref().unwrap().borrow().left) - height(&node.as_ref().unwrap().borrow().right)
}

// Check heights of left and right node, take the larger one and adds 1.
fn update_height<T>(node: &Option<Rc<RefCell<Node<T>>>>) {
    let height = std::cmp::max(height(&node.as_ref().unwrap().borrow().left), height(&node.as_ref().unwrap().borrow().right)) + 1;
    node.as_ref().unwrap().borrow_mut().height = height;
}

// Rotates the given node to the left
fn rotate_left<T>(node: Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
    // maps are used in case the value is none on the right side.
    let right = node.borrow().right.as_ref().unwrap().clone();
    let left = right.borrow().left.as_ref().map(|node| node.clone());
    node.borrow_mut().right = left.clone();
    right.borrow_mut().left = Some(node.clone());
    update_height(&right.borrow().left);
    right
}

// Rotates the given node to the right
fn rotate_right<T>(node: Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
    // maps are used in case the value is none on the left side
    let left = node.borrow().left.as_ref().unwrap().clone();
    let right = left.borrow().right.as_ref().map(|node| node.clone());
    node.borrow_mut().left = right.clone();
    left.borrow_mut().right = Some(node.clone());
    update_height(&left.borrow().right);
    left
}
// Rotates the given node left and then right
fn rotate_left_right<T>(node: Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
    let left = node.borrow().left.as_ref().unwrap().clone();
    let new_left = rotate_left(left.clone());
    node.borrow_mut().left = Some(new_left.clone());
    rotate_right(node)
}

// Rotates the given node right and then left
fn rotate_right_left<T>(node: Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
    let right = node.borrow().right.as_ref().unwrap().clone();
    let new_right = rotate_right(right.clone());
    node.borrow_mut().right = Some(new_right.clone());
    rotate_left(node)
}

// https://www.youtube.com/watch?v=vRwi_UcZGjU for explanation on rotation balance logic
fn rebalance<T: Clone>(node: Rc<RefCell<Node<T>>>) -> Rc<RefCell<Node<T>>> {
    // If the tree is unbalanced in one direction then rotatation will be used to self correct the AVL tree
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
        // When tree is left-heavy, perform right rotation then left rotation, otherwise just do a left rotation
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

    pub fn contains(&self, data: T) -> bool {
        let data_node = Node::new(Some(data));
        Self::contains_recursive(&self.root, data_node)
    }

    fn contains_recursive(root: &Option<Rc<RefCell<Node<T>>>>, data1: Rc<RefCell<Node<T>>>) -> bool {
        match root {
            Some(node) => {
                let node_data = node.borrow().data.clone();
                let data_data = data1.borrow().data.clone();
                if node_data == data_data {
                    true
                } else if node_data > data_data {
                    Self::contains_recursive(&node.borrow().left, data1)
                } else {
                    Self::contains_recursive(&node.borrow().right, data1)
                }
            }
            None => false,
        }
    }

    // This is the insert function that main will call. If the root is none, then the node is created as the root
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

    // This will be the function that main calls to delete a Node
    pub fn delete(&mut self, data: T) {
        let node = Node::new(Some(data));
        if let Some(root) = self.root.take() {
            let new_root = Self::delete_recursive(Rc::clone(&root), node);
            // Deletes the root if the only value is None, this will happen if the root is the only node in the tree and wants to be deleted
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
            // These will continuously traverse the tree if there continues to be children on the left, until the wanted data is found
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
                // These will continuously traverse the tree if there continues to be children on the right, until the wanted data is found
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
            // The current node will be replaced with the lowest value node that is on the right side. Therefore find_min always looks at the right side of the Tree
            // Note: Other versions of AVL tree will find the highest value node on the left side. The result may differ because of this
            let _ = &root.replace_with(|old| 
                match old {
                    // Case 1: Node has no children
                    // The data is set to None so that the parent can see its child has no value and delete it
                    Node{data, height, left: None, right: None} => {
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
                        // If the child that is being taken has no children then copy the value and set child to None
                        if y.borrow().left.clone().is_none() && y.borrow().right.clone().is_none(){
                            let temp = Node {
                                data: min.borrow().data.clone(),
                                height: height.clone(),
                                left: Some(Rc::clone(x)),
                                right: None,
                            }; temp
                        // Otherwise, copy the value and delete the other child
                        } else {
                            let temp = Node {
                                data: min.borrow().data.clone(),
                                height: height.clone(),
                                left: Some(Rc::clone(x)),
                                right: Some(Rc::clone(y)),
                            };
                            Self::delete_recursive(y.clone(), min);
                            temp
                        }
                    },
                    // Case 3: Node has no left child
                    Node{data, height, left: None, right: Some(y)} => {
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

    // Uses recursion to print out he in order traversal of the AVL tree by traversing through the tree to the left first, then the right
    fn in_order_traversal_recursive(node: Option<&Rc<RefCell<Node<T>>>>) {
        if let Some(n) = node {
            let node_borrow = n.borrow();
            Self::in_order_traversal_recursive(node_borrow.left.as_ref());
            println!("{:?}", node_borrow.data);
            Self::in_order_traversal_recursive(node_borrow.right.as_ref());
        }
    }

    // This will be the function that main calls to print the structure of the tree
    pub fn print_structure(&self) {
        if let Some(n) = &self.root {
            Self::print_node(&n.borrow().right,  0);
            println!("{:?}", n.borrow().data.clone().unwrap());
            Self::print_node(&n.borrow().left,  0);
        }
    }

    // This uses preorder traversal to print out the tree with structure, where the height of the tree is the widge that is used to space out the strucutre. 
    fn print_node(node: &Option<Rc<RefCell<Node<T>>>>, node_height: i32) {
        if let Some(n) = node {
            Self::print_node(&n.borrow().right, node_height + 1);
            println!("{:>width$}|-----{:?}", "", n.borrow().data.clone().unwrap(), width = ((node_height) * 7) as usize);
            Self::print_node(&n.borrow().left, node_height + 1);
            
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
