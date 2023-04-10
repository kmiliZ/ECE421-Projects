use crate::{
    db::DB,
    schema::{CreateGameSchema, FilterOptions},
    WebResult,
};
use warp::{http::StatusCode, reject, reply::json, reply::with_status, Reply};
use common::GenericResponse;

pub async fn health_checker_handler() -> WebResult<impl Reply> {
    const MESSAGE: &str = "Build CRUD API with Rust and MongoDB";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(json(response_json))
}

pub async fn games_list_handler(opts: FilterOptions, db: DB) -> WebResult<impl Reply> {
    let limit = opts.limit.unwrap_or(10) as i64;
    let page = opts.page.unwrap_or(1) as i64;

    let result_json = db
        .fetch_games(limit, page)
        .await
        .map_err(|e| reject::custom(e))?;

    Ok(json(&result_json))
}

pub async fn all_games_list_handler(db: DB) -> WebResult<impl Reply> {
    let result_json = db
        .fetch_all_games()
        .await
        .map_err(|e| reject::custom(e))?;

    Ok(json(&result_json))
}

pub async fn create_game_handler(body: CreateGameSchema, db: DB) -> WebResult<impl Reply> {
    let game = db.create_game(&body).await.map_err(|e| reject::custom(e))?;

    Ok(with_status(json(&game), StatusCode::CREATED))
}

pub async fn get_game_handler(id: String, db: DB) -> WebResult<impl Reply> {
    let game = db.get_game(&id).await.map_err(|e| reject::custom(e))?;

    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("Game with ID: {} not found", id),
    };

    if game.is_none() {
        return Ok(with_status(json(&error_response), StatusCode::NOT_FOUND));
    }

    Ok(with_status(json(&game), StatusCode::OK))
}

pub async fn delete_game_handler(id: String, db: DB) -> WebResult<impl Reply> {
    let result = db.delete_game(&id).await.map_err(|e| reject::custom(e))?;

    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("Game with ID: {} not found", id),
    };

    if result.is_none() {
        return Ok(with_status(json(&error_response), StatusCode::NOT_FOUND));
    }

    Ok(with_status(json(&""), StatusCode::NO_CONTENT))
}