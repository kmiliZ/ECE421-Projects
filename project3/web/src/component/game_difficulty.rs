pub enum GameDifficulty {
    VeryEasy,
    Easy,
    Medium,
    Hard,
    Impossible,
}

impl GameDifficulty {
    pub fn get_string(&self) -> String {
        match self {
            Self::VeryEasy => "Very Easy".to_string(),
            Self::Easy => "Easy".to_string(),
            Self::Medium => "Medium".to_string(),
            Self::Hard => "Hard".to_string(),
            Self::Impossible => "Impossible".to_string(),
        }
    }

    pub fn get_depth_level(&self) -> i32 {
        match self {
            GameDifficulty::VeryEasy => 2,
            GameDifficulty::Easy => 4,
            GameDifficulty::Medium => 6,
            GameDifficulty::Hard => 8,
            GameDifficulty::Impossible => 10,
        }
    }
}
