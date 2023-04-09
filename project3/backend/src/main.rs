mod connect4;
mod toot_and_otto;
use crate::connect4::State;

use crate::toot_and_otto::State as OtherState;
use std::io;

fn connect4_2_player(player1_name: String, player2_name: String) {
    use std::io::{stdin, stdout, Write};
    let mut board = connect4::Board::new(player1_name, player2_name, 0, false, 6, 7);

    while board.state == State::Running {
        board.display();

        // Checking who's turn it is
        if board.current_turn == 'X' {
            println!("{}'s turn", board.player1);
        } else {
            println!("{}'s turn", board.player2);
        }
        println!("Enter column (1-{}): ", board.cols);

        // Getting input from user
        while true {
            let mut col = get_input(1, board.cols.try_into().unwrap());

            if board.grid.insert_chip(col - 1, board.current_turn) != -1 {
                break;
            };
            println!("That column is full");
        }

        // Checking for win or draw
        if board.check_win() {
            println!("{} wins", board.winner);
            board.display();

            println!("Play again?");
            let mut selection = String::new();
            io::stdin()
                .read_line(&mut selection)
                .expect("Did not enter a correct string");

            if selection.trim() == "y" || selection.trim() == "yes" {
                board.restart();
            }
        } else if board.check_draw() {
            println!("Game has ended in a draw!");
            board.display();

            println!("Play again?");
            let mut selection = String::new();
            io::stdin()
                .read_line(&mut selection)
                .expect("Did not enter a correct string");

            if selection.trim() == "y" || selection.trim() == "yes" {
                board.restart();
            }
        // Switches turns if there is no win or draw
        } else {
            if board.current_turn == 'X' {
                board.current_turn = 'O';
            } else {
                board.current_turn = 'X';
            }
        }
    }
}

fn connect4_computer(player1_name: String, difficulty: i32) {
    use std::io::{stdin, stdout, Write};
    let mut board = connect4::Board::new(
        player1_name.trim().to_string(),
        "Computer".to_string(),
        difficulty,
        true,
        6,
        7,
    );

    while board.state == State::Running {
        board.display();

        if board.current_turn == 'X' {
            // Player turn
            println!("{}'s turn", board.player1);
            println!("Enter column (1-{}): ", board.cols);

            while true {
                let mut col = get_input(1, board.cols.try_into().unwrap());

                if board.grid.insert_chip(col - 1, board.current_turn) != -1 {
                    break;
                };
                println!("That column is full");
            }
        } else {
            // Computer's turn
            println!("{}'s turn", board.player2);
            let (pruning_value, best_col) =
                board.alpha_beta(board.current_turn, i32::MIN, i32::MAX, board.ai_depth);
            board
                .grid
                .insert_chip(best_col.try_into().unwrap(), board.current_turn);
        }

        // Checking for wins or draw
        if board.check_win() {
            println!("{} wins", board.winner);
            board.display();

            println!("Play again?");
            let mut selection = String::new();
            io::stdin()
                .read_line(&mut selection)
                .expect("Did not enter a correct string");

            if selection.trim() == "y" || selection.trim() == "yes" {
                board.restart();
            }
        } else if board.check_draw() {
            println!("Game has ended in a draw!");
            board.display();

            println!("Play again?");
            let mut selection = String::new();
            io::stdin()
                .read_line(&mut selection)
                .expect("Did not enter a correct string");

            if selection.trim() == "y" || selection.trim() == "yes" {
                board.restart();
            }
        } else {
            // Switching turns if the there is no win or draw
            if board.current_turn == 'X' {
                board.current_turn = 'O';
            } else {
                board.current_turn = 'X';
            }
        }
    }
}

fn toot_and_otto_2_player(player1_name: String, player2_name: String) {
    use std::io::{stdin, stdout, Write};
    let mut board = toot_and_otto::Board::new(player1_name, player2_name, 0, false, 6, 7);

    while board.state == toot_and_otto::State::Running {
        board.display();

        // Checking who's turn it is
        if board.current_turn == 'T' {
            println!("{}'s turn", board.player1);
        } else {
            println!("{}'s turn", board.player2);
        }
        println!("Enter column (1-7): ");

        // Getting input from user
        while true {
            let mut col = get_input(1, board.cols.try_into().unwrap());
            println!("col: {}", col);

            println!("Would you like to insert: ");
            println!("1. T");
            println!("2. O");

            let mut token = get_input(1, 2);

            let mut insert = '_';

            if token == 1 {
                insert = 'T';
            } else {
                insert = 'O';
            }

            // Inserting the token into the grid
            if board.grid.insert_chip(col - 1, insert) != -1 {
                break;
            };
            println!("That column is full");
        }

        // Checking for win or draw
        if board.check_win_toot() || board.check_win_otto() {
            println!("{} wins", board.winner);
            board.display();

            println!("Play again?");
            let mut selection = String::new();
            io::stdin()
                .read_line(&mut selection)
                .expect("Did not enter a correct string");

            if selection.trim() == "y" || selection.trim() == "yes" {
                board.restart();
            }
        } else if board.check_draw() {
            println!("Game has ended in a draw!");
            board.display();

            println!("Play again?");
            let mut selection = String::new();
            io::stdin()
                .read_line(&mut selection)
                .expect("Did not enter a correct string");

            if selection.trim() == "y" || selection.trim() == "yes" {
                board.restart();
            }
        // Switches turns if there is no win or draw
        } else {
            if board.current_turn == 'T' {
                board.current_turn = 'O';
            } else {
                board.current_turn = 'T';
            }
        }
    }
}

fn toot_and_otto_computer(player1_name: String, difficulty: i32) {}

// Gets a uszie input from the user in the upper and lower bounds given
fn get_input(lower_bound: usize, upper_bound: usize) -> usize {
    use std::io::{stdin, stdout, Write};
    let mut token = String::new();
    let mut temp = 0;
    while true {
        token.clear();
        stdin()
            .read_line(&mut token)
            .expect("Did not enter a correct string");
        let temp: usize = match token.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number");
                continue;
            }
        };

        if temp < lower_bound || temp > upper_bound {
            println!(
                "Please enter a number more than {} and less than {}",
                lower_bound, upper_bound
            );
            continue;
        } else {
            return temp;
        }
    }
    temp
}

fn main() {
    println!("Welcome to the Connect 4 and Toot and Otto Project");
    println!("Please indicate what kind of game you would like to play: ");
    println!("1. Connect 4");
    println!("2. Toot and Otto");
    println!("3. Exit");

    // Get the user's choice
    let mut choice = get_input(1, 3);

    match choice {
        1 => {
            println!("Would you like to play against computer or another player?");
            println!("1. Computer");
            println!("2. Another Player");

            // Get the user's choice2
            let mut choice2 = get_input(1, 2);

            match choice2 {
                1 => {
                    println!("Please enter player's name: ");
                    let mut player1_name = String::new();
                    io::stdin()
                        .read_line(&mut player1_name)
                        .expect("Failed to read line");

                    println!("What difficulty would you like?");
                    println!("1. Very Easy");
                    println!("2. Easy");
                    println!("3. Medium");
                    println!("4. Hard");
                    println!("5. Impossible");

                    let mut difficulty = get_input(1, 5);

                    match difficulty {
                        1 => {
                            connect4_computer(player1_name, 2);
                        }
                        2 => {
                            connect4_computer(player1_name, 4);
                        }
                        3 => {
                            connect4_computer(player1_name, 6);
                        }
                        4 => {
                            connect4_computer(player1_name, 8);
                        }
                        5 => {
                            connect4_computer(player1_name, 10);
                        }
                        _ => {
                            println!("Invalid option");
                            return;
                        }
                    }
                }
                2 => {
                    println!("Please enter player 1's name: ");
                    let mut player1_name = String::new();
                    io::stdin()
                        .read_line(&mut player1_name)
                        .expect("Failed to read line");

                    println!("Please enter player 2's name: ");
                    let mut player2_name = String::new();
                    io::stdin()
                        .read_line(&mut player2_name)
                        .expect("Failed to read line");

                    // Trim is used to remove the newline character
                    connect4_2_player(
                        player1_name.trim().to_string(),
                        player2_name.trim().to_string(),
                    );
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
            let mut choice2 = get_input(1, 2);

            match choice2 {
                1 => {
                    // TODO AI TOOT AND OTTO
                }
                2 => {
                    println!("Please enter player 1's name: ");
                    let mut player1_name = String::new();
                    io::stdin()
                        .read_line(&mut player1_name)
                        .expect("Failed to read line");

                    println!("Please enter player 2's name: ");
                    let mut player2_name = String::new();

                    io::stdin()
                        .read_line(&mut player2_name)
                        .expect("Failed to read line");

                    // Trim is used to remove the newline character
                    toot_and_otto_2_player(
                        player1_name.trim().to_string(),
                        player2_name.trim().to_string(),
                    );
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
