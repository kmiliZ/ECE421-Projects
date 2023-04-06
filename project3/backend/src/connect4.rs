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
        if with_ai{
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
        // Check for horizontal win
        for row in 0..self.rows {
            for col in 0..self.cols - 3 {
                if self.grid.get(row, col) == self.current_turn
                    && self.grid.get(row, col + 1) == self.current_turn
                    && self.grid.get(row, col + 2) == self.current_turn
                    && self.grid.get(row, col + 3) == self.current_turn
                {
                    if self.current_turn == 'X'{
                        self.set_winner(self.player1.clone());
                    } else {
                        self.set_winner(self.player2.clone());
                    }
                    self.state = State::Done;
                    return true;
                }
            }
        }

        // Check for vertical win
        for row in 0..self.rows - 3 {
            for col in 0..self.cols {
                if self.grid.get(row, col) == self.current_turn
                    && self.grid.get(row + 1, col) == self.current_turn
                    && self.grid.get(row + 2, col) == self.current_turn
                    && self.grid.get(row + 3, col) == self.current_turn
                {
                    if self.current_turn == 'X'{
                        self.set_winner(self.player1.clone());
                    } else {
                        self.set_winner(self.player2.clone());
                    }
                    self.state = State::Done;
                    return true;
                }
            }
        }

        // Check for diagonal win (top-left to bottom-right)
        for row in 0..self.rows - 3 {
            for col in 0..self.cols - 3 {
                if self.grid.get(row, col) == self.current_turn
                    && self.grid.get(row + 1, col + 1) == self.current_turn
                    && self.grid.get(row + 2, col + 2) == self.current_turn
                    && self.grid.get(row + 3, col + 3) == self.current_turn
                {
                    if self.current_turn == 'X'{
                        self.set_winner(self.player1.clone());
                    } else {
                        self.set_winner(self.player2.clone());
                    }
                    self.state = State::Done;
                    return true;
                }
            }
        }

        // Check for diagonal win (bottom-left to top-right)
        for row in 3..self.rows {
            for col in 0..self.cols - 3 {
                if self.grid.get(row, col) == self.current_turn
                    && self.grid.get(row - 1, col + 1) == self.current_turn
                    && self.grid.get(row - 2, col + 2) == self.current_turn
                    && self.grid.get(row - 3, col + 3) == self.current_turn
                {
                    if self.current_turn == 'X'{
                        self.set_winner(self.player1.clone());
                    } else {
                        self.set_winner(self.player2.clone());
                    }
                    self.state = State::Done;
                    return true;
                }
            }
        }

        false
    }

    pub fn check_draw(&mut self) -> bool{
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

    pub fn set_winner(&mut self, winner: String){
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
                return 1;
            // Computer won
            } else if self.winner == self.player2 {
                self.winner.clear();
                self.state = State::Running;
                return -1;
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

    // Drops a piece that is of a specific player
    pub fn drop_piece(&mut self, col: usize, player: char) {
        let mut row = self.rows - 1;
        while self.grid.get(row, col) != '_' {
            row -= 1;
        }
        self.grid.set(row, col, player);
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

    pub fn alpha_beta(&mut self, player: char, mut alpha: i32, mut beta: i32, ply: i32) -> (i32, i32, i32) {
        // check if the board is at a finished state
        if self.is_terminal() {
            return (self.game_value(), 0, 1);
        } else if ply == 0 {
            // if the algorithm runs out of depth, return zeros except for the number of expansions
            // as this is the base case of recursion; similar to a terminal state
            return (0, 0, 1);
        }

        // initialize the two other return variables
        let mut optimal_move = 0;
        let mut total_expansions = 1;

        // for the maximizing player
        if player == 'X' {
            let mut value = i32::MIN; // start at the worst case value for the maximizing player

            // go through all available moves
            for state in self.get_legal_moves() {
                // make the move
                self.drop_piece(state, player);
                // search at 1 greater depth
                let (new_value, _, new_expansions) = self.alpha_beta('O', alpha, beta, ply - 1);
                total_expansions += new_expansions;

                // change the game value if the search returned a better option
                if new_value > value {
                    value = new_value;
                    optimal_move = state; // keep track of the optimal move
                }
                // undo the move to preserve board state
                self.undo_move(state);

                // check the pruning condition
                if value >= beta {
                    break; // goes straight to return
                }
                // update alpha
                alpha = alpha.max(value);
            }

            return (value, optimal_move.try_into().unwrap(), total_expansions);
        }
        // for the minimizing player
        else {
            let mut value = i32::MAX; // start at the worst case value for the minimizing player

            // go through all available moves
            for state in self.get_legal_moves() {
                // make the move
                self.drop_piece(state, player);
                // search at 1 greater depth
                let (new_value, _, new_expansions) = self.alpha_beta('X', alpha, beta, ply - 1);
                total_expansions += new_expansions;

                // change the game value if the search returned a better option
                if new_value < value {
                    value = new_value;
                    optimal_move = state; // keep track of the optimal move
                }
                // undo the move to preserve the board state
                self.undo_move(state);

                // check the pruning condition
                if value <= alpha {
                    break; // goes straight to return
                }
                // update beta
                beta = beta.min(value);
            }

            return (value, optimal_move.try_into().unwrap(), total_expansions);
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

fn main() {

}