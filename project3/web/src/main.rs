// https://github.com/yewstack/yew/blob/master/examples/router/src/main.rs for learning how to use yew router and create nav bar
use yew::prelude::*;
use yew_router::prelude::*;
// mod cell;
// mod cell_toot;
// mod connect4_computer;
// mod connect_4;
mod MainPage;
// mod game_history;
mod howToPlayConnect4;
mod howToPlayTOOT;
// mod scoreboard;
// mod toot_otto;
// mod toot_otto_computer;
use yew::html::Scope;
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
pub enum Msg {
    ToggleNavbar,
}
pub struct App {
    navbar_active: bool,
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
            <div class="row">
                <div>
                { self.side_nav() }

                </div>
                <div class="w3-main" style="margin-left:390px;margin-right:40px">
                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main></div>
            </div>



            </BrowserRouter>

        }
    }
}
impl App {
    fn side_nav(&self) -> Html {
        html! {
            <div>
            <nav class="w3-sidenav w3-sand w3-collapse w3-top w3-large w3-padding" style="z-index:3;width:350px;font-weight:bold" id="mySidenav"><br/>
                    <a href="javascript:void(0)" class="w3-padding-xlarge w3-hide-large w3-display-topleft w3-hover-white" style="width:100%">{"Close Menu"}</a>
                    <div class="w3-container">
                      <h3 class="w3-padding-64"><b>{"Play"}<br/>{"Connect4 / TOOT-OTTO"}</b></h3>
                      <Link<Route>  to={Route::MainPage}>
                      { "Home" }
                  </Link<Route>>
                    </div>
                    <Link<Route>  to={Route::HowToPlayConnect4}>
                                    { "How to Play Connect4" }
                                </Link<Route>>
                                <Link<Route>  to={Route::connect4}>
                                { "play connect4 vs human" }
                            </Link<Route>>
                            <Link<Route>  to={Route::connect4_computer}>
                                { "play connect4 vs computer" }
                            </Link<Route>>
                    <br/>
                    <Link<Route>  to={Route::howToPlayTOOT}>
                                    { "How to Play TOOT-OTTO" }
                                </Link<Route>>
                                <Link<Route>  to={Route::toot_otto}>
                                    { "play TOOT-OTTO vs human" }
                                </Link<Route>>
                                <Link<Route>  to={Route::toot_otto_computer}>
                                    { "play TOOT-OTTO vs computer" }
                                </Link<Route>>
                    <br/>
                    <Link<Route>  to={Route::scoreboard}>
                                    { "Scoreboard" }
                                </Link<Route>>
                                <Link<Route>  to={Route::game_history}>
                                    { "Game History" }
                                </Link<Route>>
                </nav>

            </div>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::connect4 => html! {
            // <connect_4::connect_4 />
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
            <howToPlayTOOT::HowToPlayToot />
        },
        Route::HowToPlayConnect4 => html! {
            <howToPlayConnect4::HowToPlayConnect4 />
        },
        Route::MainPage => html! {
            <MainPage::MainPage />
        },
    }
}

fn main() {
    // wasm_logger::init(wasm_logger::Config::default());
    // log::trace!("Initializing yew...");
    yew::start_app::<App>();
}
