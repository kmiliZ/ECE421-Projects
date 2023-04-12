pub enum Player {
    Player1,
    Player2,
}

impl Player {
    pub fn to_char_connect4(&self) -> char {
        match self {
            Player::Player1 => 'X',
            Player::Player2 => 'O',
        }
    }

    pub fn to_char_toototto(&self) -> char {
        match self {
            Player::Player1 => 'T',
            Player::Player2 => 'O',
        }
    }

    pub fn to_string(&self, player1_name: String, player2_name: String) -> String {
        match self {
            Player::Player1 => player1_name,
            Player::Player2 => player2_name,
        }
    }

    pub fn get_color(&self) -> String {
        match self {
            Player::Player1 => "#FC2947".to_string(),
            Player::Player2 => "#00B7FF".to_string(),
        }
    }
}
