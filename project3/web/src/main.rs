use yew::functional::*;
use yew::html::Scope;
use yew::prelude::*;
use yew_router::prelude::*;
mod howToPlayConnect4;
struct Model {
    value: i64,
}

pub enum Msg {
    ToggleNavbar,
}

pub struct App {
    is_navbar_open: bool,
}
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
    howToPlayConnect4,
    #[at("/")]
    Home,
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            is_navbar_open: false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            // <BrowserRouter>
            // <Switch<Route> render={switch} />
            // </BrowserRouter>
            // display main_page her
            // {self.main_page()}
            // {self.side_nav()}
            // <span aria-hidden="true"></span>
            //             <span aria-hidden="true"></span>
            //             <span aria-hidden="true"></span>
            </div>

        }
    }
}

impl App {
    fn main_page(&self) -> Html {
        html! {
        <div>
            <form ng-submit="Game()">
                <div class="w3-container" id="services" style="margin-top:75px">
                <h5 class="w3-xxxlarge w3-text-red">
                <b>{"Welcome!"}</b>
                </h5>
                <hr style="width:50px;border:5px solid red" class="w3-round"/>
                <p>{"This application contains the following two board games, both in human Vs. human and human Vs. Computer versions."}
                </p>
                <ul>

                    <li>{"Connect 4"}</li>

                    <li>{"TOOT-OTTO"}</li>


                </ul>
                <p>{"Select the game of your choice from the side bar, and start playing. Enjoy!"}</p>
        </div>
            </form>
        </div>
        }
    }
    fn side_nav(&self) -> Html {
        html! {
            <div>
            <nav class="w3-sidenav w3-red w3-collapse w3-top w3-large w3-padding" style="z-index:3;width:350px;font-weight:bold" id="mySidenav"><br/>
                    <a href="javascript:void(0)" class="w3-padding-xlarge w3-hide-large w3-display-topleft w3-hover-white" style="width:100%">{"Close Menu"}</a>
                    <div class="w3-container">
                      <h3 class="w3-padding-64"><b>{"Play"}<br/>{"Connect4 / TOOT-OTTO"}</b></h3>
                    </div>
                    // <RouterAnchor<Route> route={Route::howToPlayConnect4}> {"Scores"} </RouterAnchor<Route>>
                    <a href="#/Connect4Computer" class="w3-padding w3-hover-white">{"Play Connect4 With Computer"}</a>
                    <a href="#/Connect4Human" class="w3-padding w3-hover-white">{"Play Connect4 with Another Human"}</a>
                    <br/>
                    <a href="#/HowToToot" class="w3-padding w3-hover-white">{"How to Play TOOT-OTTO"}</a>
                    <a href="#/TootOttoComputer" class="w3-padding w3-hover-white">{"Play Toot-Otto With Computer"}</a>
                    <a href="#/TootOttoHuman" class="w3-padding w3-hover-white">{"Play Toot-Otto With Another Human"}</a>
                    <br/>
                    <a href="#/ScoreBoard" class="w3-padding w3-hover-white">{"View Game History"}</a>
                    <a href="#/Scores" class="w3-padding w3-hover-white">{"Score Board"}</a>
                </nav>

            </div>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::connect4 => html! {},
        Route::toot_otto => html! {},
        Route::connect4_computer => html! {},
        Route::toot_otto_computer => html! {},
        Route::game_history => html! {},
        Route::scoreboard => html! {},
        Route::howToPlayTOOT => html! {},
        Route::howToPlayConnect4 => html! {
            <howToPlayConnect4::HowToPlayConnect4 />
        },
        Route::Home => html! {},
    }
}

// #[function_component(App)]
// fn app() -> Html {
//     let state = use_state(|| Model { value: 0 });

//     let onclick = {
//         let state = state.clone();

//         Callback::from(move |_| {
//             state.set(Model {
//                 value: state.value + 1,
//             })
//         }) // should not have a semi colon
//     };

// }

fn main() {
    yew::start_app::<App>();
}
