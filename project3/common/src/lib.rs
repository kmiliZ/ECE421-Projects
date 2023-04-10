use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Deserialize)]
pub struct GameResponse {
    pub gameID: String,
    pub gameType: String,
    pub player1: String,
    pub player2: String,
    pub winner: String,
    pub playedTime: DateTime<Utc>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct GameData {
    pub game: GameResponse,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct SingleGameResponse {
    pub status: String,
    pub game: GameData,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct GameListResponse {
    pub status: String,
    pub results: usize,
    pub games: Vec<GameResponse>,
}