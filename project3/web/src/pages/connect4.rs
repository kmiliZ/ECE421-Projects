#[path = "../controller/canvas_controller.rs"]
mod canvas_controller;
use backend::connect4::Board;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::{events::Event, html, Component, Context, Html};
pub struct Connect4 {
    board: Rc<RefCell<Board>>,
    is_active: bool,
    player1Name: String,
    player2Name: String,
}

pub enum Msg {
    Start,
    SetPlayer1Name(String),
    SetPlayer2Name(String),
}
impl Component for Connect4 {
    type Message = Msg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Connect4 {
            board: Rc::new(RefCell::new(Board::new(
                "".to_string(),
                "".to_string(),
                0,
                false,
                6,
                7,
            ))),
            is_active: false,
            player1Name: "".to_string(),
            player2Name: "".to_string(),
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Start => {
                self.is_active = true;
                self.board.borrow_mut().restart();
                return true;
            }
            Msg::SetPlayer1Name(input) => {
                self.player1Name = input;
                return true;
            }
            Msg::SetPlayer2Name(input) => {
                self.player2Name = input;
                return true;
            }
        }
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
            <div class="w3-container" id="services" style="margin-top:75px">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Game Started"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round"/>
            </div>
        } else {
            <div class="w3-container" id="services" style="margin-top:75px">
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
                <div >
                    <br/>
                    <h4>{"New Game:"}{&self.player1Name}{" VS "}{&self.player2Name}</h4>
                    <small>{"Disc Colors:"} {&self.player1Name} <b>{" - Red"}</b>    {" and "}    {&self.player2Name} <b>{" - Yellow</b>"}</b></small>
                    <br/>
                </div>
                <canvas id="gameboard" height="480" width="640"></canvas>
            </div>
        </div>

               }
    }
}
