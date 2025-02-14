use crate::api;
use common::GameResponse;
use gloo::console::*;
use yew::prelude::*;
use yew::{html, Component, Html};

pub enum Msg {
    ReceivedGameData(Vec<GameResponse>),
    ErrorReceiveGameData(String),
}

#[allow(non_snake_case)]
pub struct GameHistory {
    games: Option<Vec<GameResponse>>,
}

impl GameHistory {
    fn view_game_data(&self) -> Html {
        if let Some(ref games) = self.games {
            log!("view data has data");
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
                        <td>{game.playedTime.clone().naive_local()}</td>
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
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            match api::api_fetch_all_games().await {
                Ok(games) => Msg::ReceivedGameData(games),
                Err(err_str) => Msg::ErrorReceiveGameData(err_str),
            }
        });
        GameHistory { games: None }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ReceivedGameData(games) => {
                self.games = Some(games);
                return true;
            }
            Msg::ErrorReceiveGameData(err_str) => {
                log!(err_str);
                return false;
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div id="history-container">
            <div class="w3-container" id="services" style="margin-top:75px">
            <h5 class="w3-xxxlarge" style="color:#00B7FF"><b>{"Game History"}</b></h5>
            <hr style="width:50px;border:5px solid #00B7FF" class="w3-round"/>

                <div id="game-stream">
                <table class="data-table">
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
