#[derive(PartialEq)]
pub enum State {
    Done,
    Running,
    Busy,
    NotRunning,
}

pub struct Board {
    pub grid: Grid,
    pub current_turn: char,
    pub minimax_turn: char,
    pub player1: String,
    pub player2: String,
    pub ai_depth: i32,
    pub ai_playing: bool,
    pub rows: usize,
    pub cols: usize,
    pub winner: String,
    pub state: State,
}

impl Board {
    pub fn new(player1_name: String, player2_name: String, max_depth: i32, with_ai: bool, rows_input: usize, cols_input: usize) -> Board {

        let mut board = Board {
            grid: Grid::new(rows_input, cols_input),
            current_turn: 'X',
            minimax_turn: 'O',
            player1: player1_name,
            player2: player2_name,
            ai_depth: max_depth,
            ai_playing: false,
            rows: rows_input,
            cols: cols_input,
            winner: String::new(),
            state: State::Running,
        };
        if with_ai {
            board.player2 = "Computer".to_string();
            board.ai_playing = true;
        }

        board
    }

    pub fn display(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                print!("{} ", self.grid.get(row, col));
            }
            println!();
        }
    }

    pub fn check_win(&mut self) -> bool {
        let player_x = 'X';
        let player_o = 'O';

        // Check for horizontal win
        for row in 0..self.rows {
            for col in 0..self.cols - 3 {
                if self.grid.get(row, col) == player_x
                    && self.grid.get(row, col + 1) == player_x
                    && self.grid.get(row, col + 2) == player_x
                    && self.grid.get(row, col + 3) == player_x
                {
                    self.set_winner(self.player1.clone());
                    self.state = State::Done;
                    return true;
                }
            }
        }

        for row in 0..self.rows {
            for col in 0..self.cols - 3 {
                if self.grid.get(row, col) == player_o
                    && self.grid.get(row, col + 1) == player_o
                    && self.grid.get(row, col + 2) == player_o
                    && self.grid.get(row, col + 3) == player_o
                {
                    self.set_winner(self.player2.clone());

                    self.state = State::Done;
                    return true;
                }
            }
        }


        // Check for vertical win
        for row in 0..self.rows - 3 {
            for col in 0..self.cols {
                if self.grid.get(row, col) == player_x
                    && self.grid.get(row + 1, col) == player_x
                    && self.grid.get(row + 2, col) == player_x
                    && self.grid.get(row + 3, col) == player_x
                {
                    self.set_winner(self.player1.clone());
                    self.state = State::Done;
                    return true;
                }
            }
        }

        for row in 0..self.rows - 3 {
            for col in 0..self.cols {
                if self.grid.get(row, col) == player_o
                    && self.grid.get(row + 1, col) == player_o
                    && self.grid.get(row + 2, col) == player_o
                    && self.grid.get(row + 3, col) == player_o
                {

                    self.set_winner(self.player2.clone());

                    self.state = State::Done;
                    return true;
                }
            }
        }

        // Check for diagonal win (top-left to bottom-right)
        for row in 0..self.rows - 3 {
            for col in 0..self.cols - 3 {
                if self.grid.get(row, col) == player_x
                    && self.grid.get(row + 1, col + 1) == player_x
                    && self.grid.get(row + 2, col + 2) == player_x
                    && self.grid.get(row + 3, col + 3) == player_x
                {

                    self.set_winner(self.player1.clone());
                    self.state = State::Done;
                    return true;
                }
            }
        }

        for row in 0..self.rows - 3 {
            for col in 0..self.cols - 3 {
                if self.grid.get(row, col) == player_o
                    && self.grid.get(row + 1, col + 1) == player_o
                    && self.grid.get(row + 2, col + 2) == player_o
                    && self.grid.get(row + 3, col + 3) == player_o
                {
                    self.set_winner(self.player2.clone());

                    self.state = State::Done;
                    return true;
                }
            }
        }

        // Check for diagonal win (bottom-left to top-right)
        for row in 3..self.rows {
            for col in 0..self.cols - 3 {
                if self.grid.get(row, col) == player_x
                    && self.grid.get(row - 1, col + 1) == player_x
                    && self.grid.get(row - 2, col + 2) == player_x
                    && self.grid.get(row - 3, col + 3) == player_x
                {
                    self.set_winner(self.player1.clone());
                    self.state = State::Done;
                    return true;
                }
            }
        }

        for row in 3..self.rows {
            for col in 0..self.cols - 3 {
                if self.grid.get(row, col) == player_o
                    && self.grid.get(row - 1, col + 1) == player_o
                    && self.grid.get(row - 2, col + 2) == player_o
                    && self.grid.get(row - 3, col + 3) == player_o
                {

                    self.set_winner(self.player2.clone());
                    self.state = State::Done;
                    return true;
                }
            }
        }

        false
    }

    pub fn check_draw(&mut self) -> bool {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.grid.get(row, col) == '_' {
                    // Found an empty cell, the game is not a draw
                    return false;
                }
            }
        }
        // All cells are filled, the game is a draw
        self.state = State::Done;
        true
    }

    pub fn restart(&mut self) {
        self.grid = Grid::new(self.rows, self.cols);
        self.current_turn = 'X';
        self.winner.clear();
        self.state = State::Running;
    }

    pub fn set_winner(&mut self, winner: String) {
        self.winner = winner;
    }

    // Checks if the AI has found a way for the game to end
    pub fn is_terminal(&mut self) -> bool {
        let temp = self.check_draw() || self.check_win();
        self.winner.clear();
        self.state = State::Running;
        return temp;
    }

    // Returns who won in game for the alpha_beta algorithm
    pub fn game_value(&mut self) -> i32{
        if self.check_win() {
            // Player won
            if self.winner == self.player1 {
                self.winner.clear();
                self.state = State::Running;
                return -100;
            // Computer won
            } else if self.winner == self.player2 {
                self.winner.clear();
                self.state = State::Running;
                return 100;
            } else {
                // Should never reach here
                return 0;
            }
        } else {
            // No winner or draw
            return 0;
        }
    }

    // Returns all possible moves available on the board
    pub fn get_legal_moves(&self) -> Vec<usize> {
        let mut moves = Vec::new();
        for col in 0..self.cols {
            if self.grid.get(0, col) == '_' {
                moves.push(col);
            }
        }
        moves
    }

    // Returns a random move available on the board
    pub fn get_random_move(&self) -> usize {
        let mut moves = Vec::new();
        for col in 0..self.cols {
            if self.grid.get(0, col) == '_' {
                moves.push(col);
            }
        }
        if let Some(next_move) = moves.choose(&mut rand::thread_rng()) {
            return *next_move;
        } else {
            return 0;
        }
    }

    // Removes the last piece dropped at a specified column
    pub fn undo_move(&mut self, col: usize) {
        for row in 0..self.rows {
            if self.grid.get(row, col) != '_' {
                self.grid.set(row, col, '_');
                break;
            }
        }
    }

    pub fn random_walk(&mut self, player: char) -> i32 {
        // check if the board is at a win or draw, game_value tells the computer which person has won or if there was a draw
        if self.is_terminal() {
            return self.game_value();
        }

        // random computer move
        if player == 'O' {
            let col = self.get_random_move();
            self.grid.insert_chip(col, player);
            let eval = self.random_walk('X');
            self.undo_move(col);
            return eval
        } 
        // random player move
        else {
            let col = self.get_random_move();
            self.grid.insert_chip(col, player);
            let eval = self.random_walk('O');
            self.undo_move(col);
            return eval
        }
    }

    //https://medium.com/analytics-vidhya/artificial-intelligence-at-play-connect-four-minimax-algorithm-explained-3b5fc32e4a4f
    // For explaining minimax and alpha beta pruning.
    // Returns the move value, and the best column
    pub fn alpha_beta(&mut self, player: char, mut alpha: i32, mut beta: i32, ply: i32) -> (i32, i32) {
        // check if the board is at a win or draw, game_value tells the computer which person has won or if there was a draw
        if self.is_terminal() {
            return (self.game_value(), 0);
        } else if ply == 0 {
            // here the algorithm has run out of depth, which was set by the difficulty
            return (self.random_walk(player), 0);
        }

        let mut optimal_move = 0;

        // maximize computer
        if player == 'O' {
            // start at the worst case value for the maximizing computer
            let mut eval = i32::MIN; 

            // go through all available moves
            for col in self.get_legal_moves() {

                self.grid.insert_chip(col, player);
                // search at 1 more depth using recursion
                let (new_eval, _) = self.alpha_beta('X', alpha, beta, ply - 1);

                // if the result found a better col, then replace
                if new_eval > eval {
                    eval = new_eval;
                    optimal_move = col;
                }
                // undo the move to go back to original
                self.undo_move(col);

                // check the pruning condition
                if eval >= beta {
                    break;
                }
                // update alpha
                alpha = alpha.max(eval);
            }
            return (eval, optimal_move.try_into().unwrap());
        }
        // maximize player
        else if player == 'X' {
            // start at the worst case eval for the minimizing player
            let mut eval = i32::MAX; 

            // go through all available moves
            for col in self.get_legal_moves() {

                self.grid.insert_chip(col, player);
                // search at 1 more depth using recursion
                let (new_eval, _) = self.alpha_beta('O', alpha, beta, ply - 1);

                // if the result found a better col, then replace
                if new_eval < eval {
                    eval = new_eval;
                    optimal_move = col;
                }
                // undo the move to go back to original
                self.undo_move(col);

                // check the pruning condition
                if eval <= alpha {
                    break;
                }
                // update beta
                beta = beta.min(eval);
            }
            return (eval, optimal_move.try_into().unwrap());
        } else {
            //Shoould never reach here
            return (0, 0);
        }
    }


}

pub struct Grid {
    pub items: [char; 80],
    pub num_rows: usize,
    pub num_cols: usize,
}

impl Grid {
    pub fn new(rows_input: usize, cols_input: usize) -> Self {
        let mut grid = Grid {
            items: ['_'; 80],
            num_rows: rows_input,
            num_cols: cols_input,
        };
        for x in 0..(rows_input * cols_input) {
            grid.items[x] = '_';
        }
        grid
    }

    pub fn insert_chip(&mut self, col: usize, grid_val: char) -> i32 {
        // Iteratively go through each row in the column until you find the empty one starting from the bottom
        for row in (0..self.num_rows).rev() {
            match self.get(row, col) {
                '_' => {
                    self.set(row, col, grid_val);

                    return row.try_into().unwrap();
                }
                _ => {}
            }
        }
        // This means the col is full
        return -1;
    }

    pub fn get(&self, row: usize, col: usize) -> char {
        self.items[col * self.num_rows + (self.num_rows - 1 - row)]
    }

    pub fn set(&mut self, row: usize, col: usize, val: char) {
        self.items[col * self.num_rows + (self.num_rows - 1 - row)] = val;
    }
}

fn main() {}
