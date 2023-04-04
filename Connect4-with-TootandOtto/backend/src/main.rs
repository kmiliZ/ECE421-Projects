mod connect4;
mod toot_and_otto;
use connect4::ROWS;
use connect4::COLS;
use std::io;

fn connect4_2_player(){
    use std::io::{stdin,stdout,Write};
    let mut board = connect4::Board::new("p1".to_string(), "p2".to_string(), 0, false);

    while !board.game_over {
        board.display();

        println!("{}'s turn", board.current_turn);
        println!("Enter column (1-7): ");

        let mut col = String::new();
        stdin().read_line(&mut col).expect("Did not enter a correct string");

        let col: usize = match col.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if col < 1 || col > COLS {
            continue;
        }

        let mut row = ROWS - 1;
        while row > 0 && board.grid[row][col - 1] != ' ' {
            row -= 1;
        }

        board.grid[row][col - 1] = board.current_turn;

        if board.check_win() {
            println!("{} wins!", board.current_turn);
            board.display();
            board.game_over = true;
        } else {
            if board.current_turn == 'X' {
                board.current_turn = 'O';
            } else {
                board.current_turn = 'X';
            }
        }
    }
}

fn toot_and_otto_2_player(){
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
            connect4_2_player();
        }
        2 => {
            toot_and_otto_2_player();
        }
        _ => {
            return;
        }
    }
}
