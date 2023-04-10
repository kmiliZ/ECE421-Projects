// https://blog.logrocket.com/full-stack-rust-a-complete-tutorial-with-examples/
// https://codevoweb.com/build-a-crud-api-with-rust-and-mongodb/
// testing in windows: Invoke-WebRequest -Uri "http://localhost:8080/api/games" -ContentType "application/json" -Method POST -Body '{"gameType": "Mario", "player1": "p1", "player2": "p2", "winner": "p1"}'
mod db;
mod error;
mod handler;
mod schema;

use futures::future::ok;

use db::DB;
use dotenv::dotenv;
use schema::FilterOptions;
use std::convert::Infallible;
use warp::{http::Method, Filter, Rejection, fs::dir};

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "api=info");
    }
    pretty_env_logger::init();
    dotenv().ok();
    let db = DB::init().await?;

    let cors = warp::cors()
        .allow_methods(&[Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        // .allow_origins(vec!["http://localhost:3000"])
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_credentials(true);

    // all routes are defined here
    let app_router = warp::path!().and(dir("../web/dist"));
    let api_clear_router = warp::path!("api" / "clearallgames");
    let api_game_router = warp::path!("api" / "games");
    let api_game_router_id = warp::path!("api" / "games" / String);
    let api_health_checker = warp::path!("api" / "healthchecker")
        .and(warp::get())
        .and_then(handler::health_checker_handler);

    let game_routes = api_game_router
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::create_game_handler)
        .or(api_game_router
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handler::all_games_list_handler))
        .or(api_game_router
            .and(warp::get())
            .and(warp::query::<FilterOptions>())
            .and(with_db(db.clone()))
            .and_then(handler::games_list_handler))
        .or(api_clear_router
            .and(warp::delete())
            .and(with_db(db.clone()))
            .and_then(handler::delete_all_games_handler));

    let game_routes_id = api_game_router_id
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handler::get_game_handler)
        .or(api_game_router_id
            .and(warp::delete())
            .and(with_db(db.clone()))
            .and_then(handler::delete_game_handler));
    
    // adding middleware for error handling
    let routes = game_routes
    .with(warp::log("api"))
    .or(game_routes_id)
    .or(api_health_checker)
    .or(app_router)
    .with(cors)
    .recover(error::handle_rejection);

    println!("🚀 Server started successfully");
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
    Ok(())
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
