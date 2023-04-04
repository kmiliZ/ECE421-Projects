pub const ROWS: usize = 6;
pub const COLS: usize = 7;

pub struct Board {
    pub grid: [[char; COLS]; ROWS],
    pub current_turn: char,
    pub game_over: bool,
    pub player1: String,
    pub player2: String,
    pub ai_depth: u32,
    pub ai_playing: bool,
}

impl Board {
    pub fn new(player1_name: String, player2_name: String, max_depth: u32, with_ai: bool) -> Board {
        let mut board = Board {
            grid: [['_'; COLS]; ROWS],
            current_turn: 'X',
            game_over: false,
            player1: player1_name,
            player2: player2_name,
            ai_depth: max_depth,
            ai_playing: false,
        };
        if with_ai{
            board.player2 = "Computer".to_string();
            board.ai_playing = true;
        }

        board
    }

    pub fn display(&self) {
        for row in self.grid.iter() {
            println!("{}", row.iter().collect::<String>());
        }
    }

    pub fn check_win(&mut self) -> bool {
        // Check for horizontal win
        for row in 0..ROWS {
            for col in 0..COLS - 3 {
                if self.grid[row][col] == self.current_turn
                    && self.grid[row][col + 1] == self.current_turn
                    && self.grid[row][col + 2] == self.current_turn
                    && self.grid[row][col + 3] == self.current_turn
                {
                    return true;
                }
            }
        }

        // Check for vertical win
        for row in 0..ROWS - 3 {
            for col in 0..COLS {
                if self.grid[row][col] == self.current_turn
                    && self.grid[row + 1][col] == self.current_turn
                    && self.grid[row + 2][col] == self.current_turn
                    && self.grid[row + 3][col] == self.current_turn
                {
                    return true;
                }
            }
        }

        // Check for diagonal win (top-left to bottom-right)
        for row in 0..ROWS - 3 {
            for col in 0..COLS - 3 {
                if self.grid[row][col] == self.current_turn
                    && self.grid[row + 1][col + 1] == self.current_turn
                    && self.grid[row + 2][col + 2] == self.current_turn
                    && self.grid[row + 3][col + 3] == self.current_turn
                {
                    return true;
                }
            }
        }

        // Check for diagonal win (bottom-left to top-right)
        for row in 3..ROWS {
            for col in 0..COLS - 3 {
                if self.grid[row][col] == self.current_turn
                    && self.grid[row - 1][col + 1] == self.current_turn
                    && self.grid[row - 2][col + 2] == self.current_turn
                    && self.grid[row - 3][col + 3] == self.current_turn
                {
                    return true;
                }
            }
        }

        false
    }
}

fn main() {

}