use crate::pages::{
    connect4::Connect4, howToPlayConnect4::HowToPlayConnect4, howToPlayTooT::HowToPlayToot,
    mainPage::MainPage,
};
use yew::prelude::*;
use yew_router::prelude::*;
//implement yew router
#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/conntect4")]
    connect4,
    #[at("/toot_otto")]
    toot_otto,
    #[at("/connect4_computer")]
    connect4_computer,
    #[at("/toot_otto_computer")]
    toot_otto_computer,
    #[at("/ganme_history")]
    game_history,
    #[at("/scoreboard")]
    scoreboard,
    #[at("/howToPlayTOOT")]
    howToPlayTOOT,
    #[at("/howToPlayConnect4")]
    HowToPlayConnect4,
    #[at("/")]
    MainPage,
}

pub fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::connect4 => html! {
           <Connect4/>
        },
        Route::toot_otto => html! {
            // <toot_otto::toot_otto />
        },
        Route::connect4_computer => html! {
            // <connect4_computer::connect4_computer />
        },
        Route::toot_otto_computer => html! {
            // <toot_otto_computer::toot_otto_computer />
        },
        Route::game_history => html! {
            // <game_history::game_history />
        },
        Route::scoreboard => html! {
            // <scoreboard::ScoreBoard />
        },
        Route::howToPlayTOOT => html! {
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
