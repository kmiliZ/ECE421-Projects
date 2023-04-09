pub struct Board {
    pub grid: Grid,
    pub current_turn: String,
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
    pub fn new(
        player1_name: String,
        player2_name: String,
        max_depth: u32,
        with_ai: bool,
        rows_input: usize,
        cols_input: usize,
    ) -> Board {
        let mut board = Board {
            grid: Grid::new(rows_input, cols_input),
            current_turn: player1_name.clone(),
            game_over: false,
            player1: player1_name,
            player2: player2_name,
            ai_depth: max_depth,
            ai_playing: false,
            rows: rows_input,
            cols: cols_input,
            winner: String::new(),
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

    pub fn check_win_toot(&self) -> bool {
        // Check for horizontal win
        for row in 0..self.rows {
            for col in 0..self.cols - 3 {
                if self.grid.get(row, col) == 'T'
                    && self.grid.get(row, col + 1) == 'O'
                    && self.grid.get(row, col + 2) == 'O'
                    && self.grid.get(row, col + 3) == 'T'
                {
                    return true;
                }
            }
        }

        // Check for vertical win
        for row in 0..self.rows - 3 {
            for col in 0..self.cols {
                if self.grid.get(row, col) == 'T'
                    && self.grid.get(row + 1, col) == 'O'
                    && self.grid.get(row + 2, col) == 'O'
                    && self.grid.get(row + 3, col) == 'T'
                {
                    return true;
                }
            }
        }

        // Check for diagonal win (top-left to bottom-right)
        for row in 0..self.rows - 3 {
            for col in 0..self.cols - 3 {
                if self.grid.get(row, col) == 'T'
                    && self.grid.get(row + 1, col + 1) == 'O'
                    && self.grid.get(row + 2, col + 2) == 'O'
                    && self.grid.get(row + 3, col + 3) == 'T'
                {
                    return true;
                }
            }
        }

        // Check for diagonal win (bottom-left to top-right)
        for row in 3..self.rows {
            for col in 0..self.cols - 3 {
                if self.grid.get(row, col) == 'T'
                    && self.grid.get(row - 1, col + 1) == 'O'
                    && self.grid.get(row - 2, col + 2) == 'O'
                    && self.grid.get(row - 3, col + 3) == 'T'
                {
                    return true;
                }
            }
        }

        false
    }

    pub fn check_win_otto(&self) -> bool {
        // Check for horizontal win
        for row in 0..self.rows {
            for col in 0..self.cols - 3 {
                if self.grid.get(row, col) == 'O'
                    && self.grid.get(row, col + 1) == 'T'
                    && self.grid.get(row, col + 2) == 'T'
                    && self.grid.get(row, col + 3) == 'O'
                {
                    return true;
                }
            }
        }

        // Check for vertical win
        for row in 0..self.rows - 3 {
            for col in 0..self.cols {
                if self.grid.get(row, col) == 'O'
                    && self.grid.get(row + 1, col) == 'T'
                    && self.grid.get(row + 2, col) == 'T'
                    && self.grid.get(row + 3, col) == 'O'
                {
                    return true;
                }
            }
        }

        // Check for diagonal win (top-left to bottom-right)
        for row in 0..self.rows - 3 {
            for col in 0..self.cols - 3 {
                if self.grid.get(row, col) == 'O'
                    && self.grid.get(row + 1, col + 1) == 'T'
                    && self.grid.get(row + 2, col + 2) == 'T'
                    && self.grid.get(row + 3, col + 3) == 'O'
                {
                    return true;
                }
            }
        }

        // Check for diagonal win (bottom-left to top-right)
        for row in 3..self.rows {
            for col in 0..self.cols - 3 {
                if self.grid.get(row, col) == 'O'
                    && self.grid.get(row - 1, col + 1) == 'T'
                    && self.grid.get(row - 2, col + 2) == 'T'
                    && self.grid.get(row - 3, col + 3) == 'O'
                {
                    return true;
                }
            }
        }

        false
    }

    pub fn set_winner(&mut self, winner: String) {
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
                    return false;
                }
                _ => {}
            }
        }
        // This means the col is full
        return true;
    }

    pub fn get(&self, row: usize, col: usize) -> char {
        self.items[col * self.num_rows + (self.num_rows - 1 - row)]
    }

    pub fn set(&mut self, row: usize, col: usize, val: char) {
        self.items[col * self.num_rows + (self.num_rows - 1 - row)] = val;
    }
}

fn main() {}
