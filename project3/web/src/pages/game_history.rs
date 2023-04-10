use chrono::{DateTime, Utc};
use serde::Serialize;
use yew::prelude::*;
use yew::{html, Component, Html};
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct GameResponse {
    pub gameID: String,
    pub gameType: String,
    pub player1: String,
    pub player2: String,
    pub winner: String,
    pub playedTime: DateTime<Utc>,
}

#[allow(non_snake_case)]
pub struct GameHistory {
    games: Option<Vec<GameResponse>>,
}

impl GameHistory {
    fn view_game_data(&self) -> Html {
        if let Some(ref games) = self.games {
            return html! {
                {
                   games.iter().enumerate().map(|(index,game)| {
                    html! {
                        <tr id="game in games">
                        <td>{index + 1}</td>
                        <td>{game.gameType.as_str()}</td>
                        <td>{game.player1.as_str()}</td>
                        <td>{game.player2.as_str()}</td>
                        <td>{game.winner.as_str()}</td>
                        <td>{game.playedTime.clone()}</td>
                        </tr>
                    }
                   }).collect::<Html>()
                }
            };
        }
        return html! {};
    }
}

impl Component for GameHistory {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // ctx.link().send_future(asyn {
        //     match handler::g
        // })
        GameHistory { games: None }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        return true;
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id="history-container">
            <div class="w3-container" id="services" style="margin-top:75px">
            <h5 class="w3-xxxlarge w3-text-red"><b>{"Game History"}</b></h5>
            <hr style="width:50px;border:5px solid red" class="w3-round"/>

                <div id="game-stream">
                <table>
                    <tr>
                    <th>{"Game-ID"}</th>
                    <th>{"Game Type"}</th>
                    <th>{"Player1"}</th>
                    <th>{"Player2"}</th>
                    <th>{"Winner"}</th>
                    <th>{"When Played"}</th>
                      </tr>
                    {self.view_game_data()}
                </table>

                    </div>
                </div>

          </div>



        }
    }
}
