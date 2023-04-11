use std::collections::HashMap;

use crate::api;
use crate::constants::COMPUTER_NAME;
use common::GameResponse;
use gloo::console::*;
use yew::prelude::*;
use yew::{html, Component, Html};
pub enum Msg {
    ReceivedGameData(Vec<GameResponse>),
    ErrorReceiveGameData(String),
}

pub struct Score {
    games: Option<Vec<GameResponse>>,
}

impl Score {
    fn view_score_computer(&self) -> Html {
        if let Some(ref games) = self.games {
            let against_computer_count = games
                .iter()
                .filter(|x| x.player2 == COMPUTER_NAME.to_string())
                .count();
            let total_count = games.iter().count();
            let computer_win_count = games
                .iter()
                .filter(|game| game.winner == COMPUTER_NAME.to_string())
                .count();
            return html! {
                <tr id="game in games">
                    <td>{total_count}</td>
                    <td>{against_computer_count}</td>
                    <td>{computer_win_count}</td>

                </tr>
            };
        };

        return html! {};
    }

    fn view_score_computer_details(&self) -> Html {
        if let Some(ref games) = self.games {
            return html! {
                {
                   games.iter().filter(|game| game.winner == COMPUTER_NAME.to_string()).enumerate().map(|(index,game)| {
                    html! {
                        <tr id="game in games">
                        <td>{index + 1}</td>
                        <td>{game.gameType.as_str()}</td>
                        <td>{game.winner.as_str()}</td>
                        <td>{game.player1.as_str()}</td>
                        <td>{game.playedTime.clone().naive_local()}</td>
                        </tr>
                    }
                   }).collect::<Html>()
                }
            };
        }
        return html! {};
    }
    fn view_winner_score_details(&self) -> Html {
        if let Some(ref games) = self.games {
            let mut player_dict = HashMap::new();

            for game in games.iter().filter(|game| game.winner == "draw") {
                *player_dict.entry(game.player1.as_str()).or_insert(0) += 0;
                *player_dict.entry(game.player2.as_str()).or_insert(0) += 0;
            }

            for game in games.iter().filter(|game| game.winner != "draw") {
                *player_dict.entry(game.winner.as_str()).or_insert(0) += 1;
                *player_dict.entry(game.player1.as_str()).or_insert(0) += 0;
                *player_dict.entry(game.player2.as_str()).or_insert(0) += 0;
            }

            let player_vec: Vec<_> = player_dict.iter().collect();

            return html! {
                {
                    player_vec.iter().filter(|(_player,win_count)| win_count>&&0).enumerate().map(|(index,(player,win_count))| {
                    html! {
                        <tr id="game in games">
                        <td>{index + 1}</td>
                        <td>{player}</td>
                        <td>{win_count}</td>

                        </tr>
                    }
                   }).collect::<Html>()
                }
            };
        }
        return html! {};
    }
}

impl Component for Score {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            match api::api_fetch_all_games().await {
                Ok(games) => Msg::ReceivedGameData(games),
                Err(err_str) => Msg::ErrorReceiveGameData(err_str),
            }
        });
        Score { games: None }
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
        <div id="main">
            <div class="w3-container" id="services" style="margin-top:75px">
                <h5 class="w3-xxxlarge" style="color:#00B7FF"><b>{"Score Board"}</b></h5>
                <hr style="width:50px;border:5px solid #00B7FF" class="w3-round"/>

                <h4>{"Games Won by Computer"}</h4>
                <table class="data-table">
                    <tr>
                        <th>{"Total Games Played"}</th>
                        <th>{"Games Against Computer"}</th>
                        <th>{"Games Computer Won"}</th>
                    </tr>
                    {self.view_score_computer()}
            </table>
            <br/>
            <h4>{"Details of Games Won by Computer"}</h4> <div id="game-stream">
            <table class="data-table">
                <tr>
                    <th>{"Sl. No."}</th>
                    <th>{"Game Type"}</th>
                    <th>{"Winner"}</th>
                    <th>{"Played Against"}</th>
                    <th>{"When Played"}</th>
                </tr>
            {self.view_score_computer_details()}

             </table>

             <br/>


                <h4>{"Details of Games Won by All Players"}</h4>

            <div id="game-stream">
            <table class="data-table">
                <tr>
                    <th>{"Sl. No."}</th>
                    <th>{"Winner or Draw"}</th>
                    <th>{"No. of Wins<"}</th>
                  </tr>
                   {self.view_winner_score_details()}
            </table>

            </div>
        </div>
            </div>

        </div>}
    }
}
