use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub gameID: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGameSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gameType: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winner: Option<String>,
}
