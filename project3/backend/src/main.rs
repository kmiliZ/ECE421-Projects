mod connect4;
mod toot_and_otto;
use std::io;
use crate::connect4::State;

fn connect4_2_player(player1_name: String, player2_name: String){
    use std::io::{stdin,stdout,Write};
    let mut board = connect4::Board::new(player1_name, player2_name, 0, false, 6, 7);

    while board.state == State::Running {
        board.display();

        if board.current_turn == 'X'{
            println!("{}'s turn", board.player1);
        } else {
            println!("{}'s turn", board.player2);
        }
        println!("Enter column (1-7): ");

        while true {
            let mut col = String::new();
            stdin().read_line(&mut col).expect("Did not enter a correct string");

            let col: usize = match col.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input, please enter a number");
                    continue;
                }
            };

            if col < 1 || col > board.cols.try_into().unwrap() {
                println!("Please enter a number less than {}", board.cols);
                continue;
            }

            if board.grid.insert_chip(col - 1, board.current_turn){
                break;
            };
            println!("That column is full");
        }

        if board.check_win() {
            println!("{} wins", board.winner);
            board.display();

            println!("Play again?");
            let mut selection = String::new();
            io::stdin().read_line(&mut selection).expect("Did not enter a correct string");

            if selection.trim() == "y" || selection.trim() == "yes" {
                board.restart();
            }

        } else if board.check_draw() {
            println!("Game has ended in a draw!");
            board.display();

            println!("Play again?");
            let mut selection = String::new();
            io::stdin().read_line(&mut selection).expect("Did not enter a correct string");

            if selection.trim() == "y" || selection.trim() == "yes" {
                board.restart();
            }

        } else {
            if board.current_turn == 'X' {
                board.current_turn = 'O';
            } else {
                board.current_turn = 'X';
            }
        }

    }
}

fn toot_and_otto_2_player(player1_name: String, player2_name: String){
    return;
}

fn main() {
    println!("Welcome to the Connect 4 and Toot and Otto Project");
    println!("Please indicate what kind of game you would like to play: ");
    println!("1. Connect 4");
    println!("2. Toot and Otto");
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
            println!("Would you like to play against computer or another player?");
            println!("1. Computer");
            println!("2. Another Player");

             // Get the user's choice
            let mut choice2 = String::new();
            io::stdin().read_line(&mut choice2).expect("Failed to read line");

            let choice2: u32 = match choice2.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input, please enter a number");
                    return;
                }
            };
            

            match choice2 {
                1 => {
                    // TODO AI CONNECT 4
                }
                2 => {
                    println!("Please enter player 1's name: ");
                    let mut player1_name = String::new();
                    io::stdin().read_line(&mut player1_name).expect("Failed to read line");

                    println!("Please enter player 2's name: ");
                    let mut player2_name = String::new();
                    io::stdin().read_line(&mut player2_name).expect("Failed to read line");

                    // Trim is used to remove the newline character
                    connect4_2_player(player1_name.trim().to_string(), player2_name.trim().to_string());
                }
                _ => {
                    return;
                }
            }

            
        }
        2 => {
            println!("Would you like to play against computer or another player?");
            println!("1. Computer");
            println!("2. Another Player");

             // Get the user's choice
            let mut choice2 = String::new();
            io::stdin().read_line(&mut choice2).expect("Failed to read line");

            let choice2: u32 = match choice2.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input, please enter a number");
                    return;
                }
            };
            

            match choice2 {
                1 => {
                    // TODO AI TOOT AND OTTO
                }
                2 => {
                    println!("Please enter player 1's name: ");
                    let mut player1_name = String::new();
                    io::stdin().read_line(&mut player1_name).expect("Failed to read line");

                    println!("Please enter player 2's name: ");
                    let mut player2_name = String::new();
                    io::stdin().read_line(&mut player2_name).expect("Failed to read line");

                    println!("Would {} like to be Toot or Otto?: ", player1_name.trim().to_string());
                    println!("1. Toot");
                    println!("2. Otto");
                    let mut player1_choice = String::new();
                    io::stdin().read_line(&mut player1_choice).expect("Failed to read line");
                    while player1_choice != "1" || player1_choice != "2"{
                        println!("Not a valid input, please try again");
                        io::stdin().read_line(&mut player1_choice).expect("Failed to read line");
                    }


                    // Trim is used to remove the newline character
                    toot_and_otto_2_player(player1_name.trim().to_string(), player2_name.trim().to_string());
                }
                _ => {
                    return;
                }
            }
            
        }
        _ => {
            return;
        }
    }
}
