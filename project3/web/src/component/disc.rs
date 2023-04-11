pub enum DiscType {
    T,
    O,
}

impl DiscType {
    pub fn to_char(&self) -> char {
        match self {
            Self::T => 'T',
            Self::O => 'O',
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::T => "T".to_string(),
            Self::O => "O".to_string(),
        }
    }

    pub fn is_o_selected(&self) -> bool {
        match self {
            Self::T => false,
            Self::O => true,
        }
    }

    pub fn is_t_selected(&self) -> bool {
        match self {
            Self::T => true,
            Self::O => false,
        }
    }
}
