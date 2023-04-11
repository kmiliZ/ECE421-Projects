#[path = "../controller/canvas_controller.rs"]
mod canvas_controller;
use crate::api;
use backend::connect4::Board;
use gloo::console::*;
use gloo_timers::callback::Timeout;

use crate::component::game_difficulty::GameDifficulty;
use crate::component::player::Player;
use crate::constants::COMPUTER_NAME;

use std::time::Duration;
use std::{cell::RefCell, rc::Rc};
use std::{clone, thread};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::html::Scope;
use yew::virtual_dom::VNode;

use yew::{events::Event, html, Component, Context};
pub struct Connect4Computer {
    board: Rc<RefCell<Board>>,
    is_active: bool,
    player1_name: String,
    player2_name: String,
    canvas: Option<canvas_controller::Canvas>,
    canvas_id: String,
    current_player: Player,
    difficulty: GameDifficulty,
}

pub enum Msg {
    Start,
    SetPlayer1Name(String),
    InsertChip((usize, usize)),
    PostGame(String),
    PostOK,
    PostError,
    ChangeGameDifficulty(GameDifficulty),
}
impl Connect4Computer {
    fn check_win(&self) -> bool {
        if self.board.as_ref().borrow_mut().check_win() {
            return true;
        }
        false
    }

    fn check_draw(&self) -> bool {
        if self.board.as_ref().borrow_mut().check_draw() {
            return true;
        }
        false
    }

    fn stop_game(&mut self) {
        self.is_active = false;
    }

    fn draw_board(&mut self, row: usize, col: usize) {
        self.canvas
            .as_ref()
            .unwrap()
            .draw_mask("#FA9884".to_owned(), row, col, 25.0);
    }

    fn clear_canvas(&mut self) {
        self.canvas.as_ref().unwrap().clear_canvas();
    }

    fn change_current_board_turn(&mut self) {
        let player = &self.current_player;

        self.board.borrow_mut().current_turn = player.to_char();
    }

    fn computer_make_move(&mut self) -> i32 {
        log!("compter makes move");
        let (_pruning_value, best_col) = self.board.as_ref().borrow_mut().alpha_beta(
            self.current_player.to_char().clone(),
            i32::MIN,
            i32::MAX,
            self.difficulty.get_depth_level(),
        );
        return best_col;
    }

    fn insert_chip(&mut self, col: usize) -> i32 {
        return self
            .board
            .as_ref()
            .borrow_mut()
            .grid
            .insert_chip(col, self.current_player.to_char().clone());
    }
}

impl Component for Connect4Computer {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Connect4Computer {
            board: Rc::new(RefCell::new(Board::new(
                "".to_string(),
                COMPUTER_NAME.to_string(),
                0,
                false,
                6,
                7,
            ))),
            is_active: false,
            player1_name: "".to_string(),
            player2_name: COMPUTER_NAME.to_string(),
            canvas: None,
            canvas_id: "gameboard-connect4-hc".to_string(),
            current_player: Player::Player1,
            difficulty: GameDifficulty::VeryEasy,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Start => {
                if !self.is_active {
                    self.clear_canvas();
                    self.draw_board(6, 7);
                    self.is_active = true;
                    self.board.borrow_mut().restart();
                    self.current_player = Player::Player1;
                    log!("game started");
                }

                return true;
            }
            Msg::SetPlayer1Name(input) => {
                self.player1_name = input;
                return true;
            }

            Msg::InsertChip((col, _row)) => {
                let link = ctx.link().clone();

                if self.is_active {
                    let inserted_row = self.insert_chip(col);

                    let color = self.current_player.get_color().clone();
                    if inserted_row >= 0 {
                        log!("draw chip");
                        canvas_controller::animate(
                            self.canvas_id.clone(),
                            col as i64,
                            inserted_row as i64,
                            0,
                            color,
                            None,
                            self.check_win(),
                            self.check_draw(),
                            self.current_player
                                .to_string(self.player1_name.clone(), self.player2_name.clone()),
                        );

                        if self.check_win() {
                            self.is_active = false;
                            link.send_message(Msg::PostGame("".to_string()));
                            return true;
                        } else {
                            if self.check_draw() {
                                self.is_active = false;
                                link.send_message(Msg::PostGame("draw".to_string()));
                                return true;
                            }
                        }
                        // change current turn here, both board and connect4
                        match self.current_player {
                            Player::Player1 => {
                                self.current_player = Player::Player2;
                                let col: usize = self.computer_make_move().try_into().unwrap();
                                // thread::sleep(Duration::from_secs(1));
                                let timeout = Timeout::new(300, move || {
                                    log!("callback");
                                    link.send_message(Msg::InsertChip((col, 0)));
                                });
                                timeout.forget();
                            }
                            Player::Player2 => self.current_player = Player::Player1,
                        }
                        self.change_current_board_turn();
                    }
                    return true;
                }
                link.send_message(Msg::Start);
                return true;
            }
            Msg::PostGame(winner) => {
                let mut name = winner.clone();
                if name == "" {
                    match self.current_player {
                        Player::Player1 => {
                            name = self.player1_name.clone();
                        }
                        Player::Player2 => {
                            name = self.player2_name.clone();
                        }
                    }
                }

                let game_data = format!(
                    r#"{{"gameType": "{}", "player1": "{}", "player2": "{}", "winner": "{}"}}"#,
                    "Connect4",
                    self.player1_name.clone(),
                    self.player2_name.clone(),
                    name.clone()
                );
                ctx.link().send_future(async move {
                    match api::api_create_game(&game_data.clone()).await {
                        Ok(_games) => Msg::PostOK,
                        Err(_err_str) => Msg::PostError,
                    }
                });
                return false;
            }
            Msg::PostOK => false,
            Msg::PostError => false,
            Msg::ChangeGameDifficulty(level) => {
                self.difficulty = level;
                true
            }
        }
    }
    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let canvas = canvas_controller::Canvas::new(self.canvas_id.clone());
        let bounding_rect = canvas.canvas.get_bounding_client_rect();
        let rect_left = bounding_rect.left();
        let rect_top = bounding_rect.top();

        let cols = self.board.as_ref().borrow().cols;
        let rows = self.board.as_ref().borrow().rows;

        let mut clicked_on_cell = false;

        let link = ctx.link().clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            let clicked_x = (event.client_x() as f64) - rect_left;
            let clicked_y = (event.client_y() as f64) - rect_top;
            for i in 0..cols {
                let x_center = (75 * i + 100) as f64;
                if (x_center - clicked_x).abs() <= 25.0 {
                    for j in 0..rows {
                        let y_center = (75 * j + 50) as f64;
                        if (y_center - clicked_y).abs() <= 25.0 {
                            let coord = (i, j);
                            link.send_message(Msg::InsertChip(coord));
                            clicked_on_cell = true;
                            break;
                        }
                    }
                }
            }
            if !clicked_on_cell {
                link.send_message(Msg::Start);
            }
            // let coord = (event.client_x(), event.client_y());
        });
        canvas.register_onclick_listener(closure);
        self.canvas = Some(canvas);
    }

    fn view(&self, ctx: &Context<Self>) -> VNode {
        let on_dangerous_change_input1 = ctx.link().callback(|e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            // You must KNOW target is a HtmlInputElement, otherwise
            // the call to value would be Undefined Behaviour (UB).
            log!("board clicked");

            Msg::SetPlayer1Name(target.unchecked_into::<HtmlInputElement>().value())
        });

        html! {
        <div id="main" >
        if self.is_active {
            <div class="w3-container" id="services" style="margin-top:45px">
                <h5 class="w3-xxlarge w3-text-red"><b>{"Game Started: "}{self.current_player.to_string(self.player1_name.clone() , self.player2_name.clone())}{" \'s turn"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round"/>
            </div>
        } else {
            <div class="w3-container" id="services" style="margin-top:45px">
                <h5 class="w3-xxlarge w3-text-red"><b>{"Enter Your Name"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round"/>
            </div>
        }

            <div class="col-md-offset-4 col-md-8">
                <form>
                    <div class="col-md-offset-3 col-md-8">
                        <input id="textbox1" type="text" placeholder="Player's Name"  disabled={self.is_active} onchange={on_dangerous_change_input1}/>
                        <input id="startbutton" class="button" type="submit" value="Start Game" disabled={self.is_active} onclick={ctx.link().callback(|_| Msg::Start)}/>
                    </div>
                </form>
                <div >
                    <br/>
                        <div class="game-difficulty">
                            {"Select The Game Difficulty:"}
                            <input type="radio" id="Very-Easy" value="Very Easy" checked={self.difficulty.get_string()=="Very Easy" } oninput = {ctx.link().callback(|_| Msg::ChangeGameDifficulty(GameDifficulty::VeryEasy))} />
                            <label for="Very-Easy">{"Very Easy"}</label>
                            <input type="radio" id="Easy" value="Easy" checked={self.difficulty.get_string()=="Easy"} oninput = {ctx.link().callback(|_| Msg::ChangeGameDifficulty(GameDifficulty::Easy))}/>
                            <label for="Easy">{"Easy"}</label>
                            <input type="radio" id="Medium" value="Medium" checked={self.difficulty.get_string()=="Medium"} oninput = {ctx.link().callback(|_| Msg::ChangeGameDifficulty(GameDifficulty::Medium))}/>
                            <label for="Medium">{"Medium"}</label>
                            <input type="radio" id="Hard" value="Hard" checked={self.difficulty.get_string()=="Hard"} oninput = {ctx.link().callback(|_| Msg::ChangeGameDifficulty(GameDifficulty::Hard))}/>
                            <label for="Hard">{"Hard"}</label>
                            <input type="radio" id="Impossible" value="Impossible" checked={self.difficulty.get_string()=="Impossible"} oninput = {ctx.link().callback(|_| Msg::ChangeGameDifficulty(GameDifficulty::Impossible))}/>
                            <label for="Impossible">{"Impossible"}</label>
                        </div>
                    <h4>{"New Game:"}{&self.player1_name}{" VS "}{&self.player2_name}</h4>
                    <h5>{"current difficulty: "} {&self.difficulty.get_string()}</h5>
                    <small>{"Disc Colors: "} {&self.player1_name} <b>{" - Red"}</b>    {" and "}    {&self.player2_name} <b>{" - Blue"}</b></small>

                </div>
                <br/>
                <canvas id={self.canvas_id.clone()} height="480" width="640"></canvas>
            </div>
        </div>

               }
    }
}
