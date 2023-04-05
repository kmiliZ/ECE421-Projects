pub struct Board {
    pub grid: Grid,
    pub current_turn: char,
    pub game_over: bool,
    pub player1: String,
    pub player2: String,
    pub ai_depth: u32,
    pub ai_playing: bool,
    pub rows: usize,
    pub cols: usize,
    pub winner: String,
}

impl Board {
    pub fn new(player1_name: String, player2_name: String, max_depth: u32, with_ai: bool, rows_input: usize, cols_input: usize) -> Board {
        let mut board = Board {
            grid: Grid::new(rows_input, cols_input),
            current_turn: 'X',
            game_over: false,
            player1: player1_name,
            player2: player2_name,
            ai_depth: max_depth,
            ai_playing: false,
            rows: rows_input,
            cols: cols_input,
            winner: String::new(),
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
                    return true;
                }
            }
        }

        false
    }

    pub fn set_winner(&mut self, winner: String){
        self.winner = winner;
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

    pub fn insert_chip(&mut self, col: usize, grid_val: char) -> bool {
        // Iteratively go through each row in the column until you find the empty one starting from the bottom
        for row in (0..self.num_rows).rev() {
            match self.get(row, col) {
                '_' => {
                    self.set(row, col, grid_val);
                    return true;
                }
                _ => {}
            }
        }
        // This means the col is full
        return false;
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