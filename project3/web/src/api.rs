use common::*;
use reqwasm::http;

pub async fn api_fetch_all_games(user_data: &str) -> Result<Vec<GameResponse>, String> {
    let response = match http::Request::get("http://localhost:8080/api/games")
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<GenericResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<GameListResponse>().await;
    match res_json {
        Ok(data) => Ok(data.games),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

// required game_data format: '{"gameType": "Connect4/TootOtto", "player1": "p1", "player2": "p2", "winner": "p1"}'
pub async fn api_create_game(game_data: &str) -> Result<GameResponse, String> {
    let response = match http::Request::post("http://localhost:8080/api/games")
        .header("Content-Type", "application/json")
        .body(game_data)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<GenericResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<SingleGameResponse>().await;
    match res_json {
        Ok(data) => Ok(data.game.game),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}