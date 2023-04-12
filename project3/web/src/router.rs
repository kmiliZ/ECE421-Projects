use crate::pages::{
    connect4::Connect4, connect4_computer::Connect4Computer, game_history::GameHistory,
    how_to_play_connect4::HowToPlayConnect4, how_to_play_toototto::HowToPlayToot,
    main_page::MainPage, score_board::Score, tootOtto_computer::TootOttoComputer,
    TootOtto::TootOtto,
};
use yew::prelude::*;
use yew_router::prelude::*;
//implement yew router
#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/conntect4")]
    Connect4,
    #[at("/toot_otto")]
    TootOtto,
    #[at("/connect4_computer")]
    Connect4Computer,
    #[at("/toot_otto_computer")]
    TootOttoComputer,
    #[at("/game_history")]
    GameHistory,
    #[at("/scoreboard")]
    Scoreboard,
    #[at("/howToPlayTOOT")]
    HowToPlayToot,
    #[at("/howToPlayConnect4")]
    HowToPlayConnect4,
    #[at("/")]
    MainPage,
}

pub fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Connect4 => html! {
           <Connect4/>
        },
        Route::TootOtto => html! {
            <TootOtto/>
        },
        Route::Connect4Computer => html! {
            <Connect4Computer/>
        },
        Route::TootOttoComputer => html! {
           <TootOttoComputer/>
        },
        Route::GameHistory => html! {
            <GameHistory/>
        },
        Route::Scoreboard => html! {
            <Score/>
        },
        Route::HowToPlayToot => html! {
            <HowToPlayToot />
        },
        Route::HowToPlayConnect4 => html! {
            <HowToPlayConnect4 />
        },
        Route::MainPage => html! {
            <MainPage />
        },
    }
}
