#[path = "../controller/canvas_controller.rs"]
mod canvas_controller;
use crate::api;
use cli::toot_and_otto::Board;
use gloo::console::*;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::virtual_dom::VNode;
use yew::{events::Event, html, Component, Context};

use crate::component::disc::DiscType;
use crate::component::player::Player;

pub struct TootOtto {
    board: Rc<RefCell<Board>>,
    is_active: bool,
    player1_name: String,
    player2_name: String,
    canvas: Option<canvas_controller::Canvas>,
    canvas_id: String,
    current_player: Player,
    disc_type: DiscType,
}

pub enum Msg {
    Start,
    SetPlayer1Name(String),
    SetPlayer2Name(String),
    InsertChip((usize, usize)),
    PostGame(String),
    PostOK,
    PostError,
}
impl TootOtto {
    fn check_win_otto(&self) -> bool {
        log!("checking win...");
        if self.board.as_ref().borrow_mut().check_win_otto() {
            return true;
        }
        false
    }

    fn check_win_toot(&self) -> bool {
        log!("checking win...");
        if self.board.as_ref().borrow_mut().check_win_toot() {
            return true;
        }
        false
    }

    fn check_win(&self) -> bool {
        return self.check_win_otto() || self.check_win_toot();
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

        self.board.borrow_mut().current_turn = player.to_char_toototto();
    }
}

impl Component for TootOtto {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        TootOtto {
            board: Rc::new(RefCell::new(Board::new(
                "".to_string(),
                "".to_string(),
                0,
                false,
                6,
                7,
            ))),
            is_active: false,
            player1_name: "".to_string(),
            player2_name: "".to_string(),
            canvas: None,
            canvas_id: "gameboard-TootOtto-hh".to_string(),
            current_player: Player::Player1,
            disc_type: DiscType::T,
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
            Msg::SetPlayer2Name(input) => {
                self.player2_name = input;
                return true;
            }
            Msg::InsertChip((col, _row)) => {
                let link = ctx.link().clone();
                if self.is_active {
                    // grab radio input value for disc type

                    let document = web_sys::window().unwrap().document().unwrap();
                    let element = document.get_element_by_id(&"input-disc-T").unwrap();
                    let input_select_t: web_sys::HtmlInputElement = element
                        .dyn_into::<web_sys::HtmlInputElement>()
                        .map_err(|_| ())
                        .unwrap();
                    let is_checked = input_select_t.checked();
                    if is_checked {
                        self.disc_type = DiscType::T;
                    } else {
                        self.disc_type = DiscType::O;
                    }
                    let inserted_row = self
                        .board
                        .as_ref()
                        .borrow_mut()
                        .grid
                        .insert_chip(col, self.disc_type.to_char().clone());

                    let color = self.current_player.get_color().clone();
                    if inserted_row >= 0 {
                        let mut winner = "".to_string();
                        if self.check_win_toot() {
                            winner = self.player1_name.clone();
                        }
                        if self.check_win_otto() {
                            winner = self.player2_name.clone()
                        }
                        canvas_controller::animate(
                            self.canvas_id.clone(),
                            col as i64,
                            inserted_row as i64,
                            0,
                            color,
                            Some(self.disc_type.to_string().clone()),
                            self.check_win(),
                            self.check_draw(),
                            winner,
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
                        // change current turn here, both board and TootOtto
                        match self.current_player {
                            Player::Player1 => self.current_player = Player::Player2,
                            Player::Player2 => self.current_player = Player::Player1,
                        }
                        self.change_current_board_turn();
                    }
                    return true;
                }
                link.send_message(Msg::Start);
                return true;
            }
            Msg::PostOK => false,
            Msg::PostError => false,
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
                    "TOOT-OTTO",
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
            log!("board clicked");
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
            Msg::SetPlayer1Name(target.unchecked_into::<HtmlInputElement>().value())
        });
        let on_dangerous_change_input2 = ctx.link().callback(|e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            // You must KNOW target is a HtmlInputElement, otherwise
            // the call to value would be Undefined Behaviour (UB).
            Msg::SetPlayer2Name(target.unchecked_into::<HtmlInputElement>().value())
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
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Your Name"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round"/>
            </div>
        }


            <div class="col-md-offset-4 col-md-8">
            <form>
                <div class="col-md-offset-3 col-md-8">
                    <input id="textbox1" type="text" placeholder="Player 1's Name"  disabled={self.is_active} onchange={on_dangerous_change_input1}/>
                    <input id="textbox2" type="text" placeholder="Player 2's Name"  disabled={self.is_active} onchange ={on_dangerous_change_input2}/>
                    <input id="startbutton" class="button" type="submit" value="Start Game" disabled={self.is_active} onclick={ctx.link().callback(|_| Msg::Start)}/>
                </div>
            </form>


            <div  >
                <br/>

                <h4>{"New Game:"}{&self.player1_name}{" VS "}{&self.player2_name}</h4>
                <small>{"Winning Combination: "} {&self.player1_name} <b>{" - TOOT"}</b>    {" and "}    {&self.player2_name} <b>{" - OTTO"}</b></small>
                <br/>
                <form>
                <h4>{"Select a Disc Type   :"}
                  <input type="radio" name="choice" value="T" id="input-disc-T" checked={self.disc_type.is_t_selected()}/> {"T"}
                  <input type="radio" name="choice" value="O" id="input-disc-O" checked={self.disc_type.is_o_selected()}/>{"O"}

           </h4>
           </form>
            </div>
            <br/>
                <canvas id={self.canvas_id.clone()} height="480" width="640"></canvas>
            </div>
        </div>

               }
    }
}
