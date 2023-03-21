use std::io;
use crate::avl::AVLTree;

fn avlTree(){
    let mut tree = AVLTree::new();
    loop {
        // Print the menu of available options
        println!("\nAVL Tree Operations (type the number):");
        println!("1. Insert a node");
        println!("2. Delete a node");
        println!("3. Count the number of leaves");
        println!("4. Return the height of the tree");
        println!("5. Print in order traversal");
        println!("6. Check if the tree is empty");
        println!("7. Print the tree structure");
        println!("8. Exit");

        // Get the user's choice
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number");
                continue;
            }
        };

        // Call the appropriate function based on the user's choice
        match choice {
            1 => {
                // Get the value to insert from the user
                println!("Enter a value to insert:");
                let mut value = String::new();
                io::stdin()
                    .read_line(&mut value)
                    .expect("Failed to read line");
                let value: i32 = match value.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input, please enter a number");
                        continue;
                    }
                };

                // Insert the value into the AVL tree
                tree.insert(value);
                println!("Value {} inserted successfully", value);
            }
            2 => {
                // Get the value to delete from the user
                println!("Enter a value to delete:");
                let mut value = String::new();
                io::stdin()
                    .read_line(&mut value)
                    .expect("Failed to read line");
                let value: i32 = match value.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input, please enter a number");
                        continue;
                    }
                };

                // Delete the value from the AVL tree
                if tree.contains(value){
                    tree.delete(value);
                    println!("{} deleted from tree", value);
                } else {
                    println!("{} does not exist in tree", value);
                }
                
            }
            3 => {
                // Count the number of leaves in the AVL tree
                let num_leaves = tree.count_leaves();
                println!("Number of leaves in the tree: {}", num_leaves);
            }
            4 => {
                // Get the height of the AVL tree
                let height = tree.get_height();
                println!("Height of the tree: {}", height);
            }
            5 => {
                // Print the AVL tree in order traversal
                println!("In order traversal of the tree:");
                tree.in_order_traversal();
            }
            6 => {
                // Check if the AVL tree is empty
                if tree.is_empty() {
                    println!("The tree is empty");
                } else {
                    println!("The tree is not empty");
                }
            }
            7 => {
                // Print the AVL tree structure
                println!("Structure of the tree:");
                tree.print_structure();
            }
            8 => {
                // Exit the program
                println!("Exiting program");
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }
    }
}

fn RBTree(){

}

pub fn interface(){
    println!("Welcome to the Tree project");
    println!("Please indicate what kind of tree you would like to build (type the number): ");
    println!("1. Red-Black Tree");
    println!("2. AVL Tree");
    println!("3. Exit");

    // Get the user's choice
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, please enter a number");
            return;
        }
    };

    match choice {
            1 => {
                RBTree();
            }
            2 => {
                avlTree();
            }
            _=> {
                return;
            }
    }
}