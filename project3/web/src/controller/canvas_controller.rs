use core::f64::consts::PI;
use gloo::console::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
// do canvas struct and implement draw_canvas, draw circle, animation and stuff
// let connect4 to have an option field for canvas_controller_object. whenver need to call canvas related,
// do canvas.unwrap? try that

const BOARD_COL: i64 = 7;
const BOARD_ROW: i64 = 6;

pub struct Canvas {
    pub canvas: web_sys::HtmlCanvasElement,
    pub context: web_sys::CanvasRenderingContext2d,
}

impl Canvas {
    pub fn new(cid: String) -> Canvas {
        //https://rustwasm.github.io/wasm-bindgen/examples/2d-canvas.html
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(&cid).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Canvas { canvas, context }
    }

    pub fn draw_circle(&self, fill_value: String, x: usize, y: usize, r: f64) {
        let x_f = (75 * x + 100) as f64;
        let y_f = (75 * y + 50) as f64;
        self.draw_circle_at(fill_value, x_f, y_f, r, None);
    }

    pub fn draw_circle_at(&self, fill_value: String, x: f64, y: f64, r: f64, text: Option<String>) {
        self.context.save();
        self.context.set_fill_style(&fill_value.into());
        let strok: String = "black".to_string();
        self.context.set_stroke_style(&strok.into());
        self.context.begin_path();

        let _ = self.context.arc(x, y, r, 0.0, 2.0 * PI);
        self.context.fill();
        self.context.restore();

        if let Some(s) = text {
            self.context.set_font("bold 25px serif");
            self.context.save();
            let strok: String = "black".to_string();
            self.context.set_fill_style(&strok.into());
            self.context.begin_path();
            let _ = self.context.fill_text(&s, x - 8.5, y + 8.0);
            self.context.restore();
        }
    }

    pub fn draw_mask(&self, fill_value: String, row: usize, col: usize, r: f64) {
        self.context.save();
        self.context.set_fill_style(&fill_value.into());
        self.context.begin_path();
        for y in 0..row {
            for x in 0..col {
                let x_f = (75 * x + 100) as f64;
                let y_f = (75 * y + 50) as f64;
                // r 25
                let _ = self.context.arc(x_f, y_f, r, 0.0, 2.0 * PI);
                self.context.rect(x_f + 50.0, y_f - 50.0, -100.0, 100.0)
            }
        }
        self.context.fill();
        self.context.restore();
    }
    pub fn register_onclick_listener(&self, onclick: Closure<dyn FnMut(web_sys::MouseEvent)>) {
        let _ = self
            .canvas
            .set_onclick(Some(onclick.as_ref().unchecked_ref()));
        onclick.forget();
    }

    pub fn clear_canvas(&self) {
        self.context.clear_rect(
            0.0,
            0.0,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        )
    }
}

pub fn draw_circle_at_canvas(id: String, fill_value: String, x: f64, y: f64, text: Option<String>) {
    let canvas = Canvas::new(id);
    canvas.draw_circle_at(fill_value, x, y, 25.0, text);
}

pub fn display_text_on_canvas(id: String, text: String, color: String) {
    let canvas = Canvas::new(id);

    let bounding_rect = canvas.canvas.get_bounding_client_rect();
    let x = bounding_rect.width() / 2.0;
    let y = bounding_rect.height() / 2.0;
    canvas.context.save();
    // canvas.context.begin_path();
    canvas.context.set_text_align("center");
    canvas.context.set_fill_style(&color.into());
    canvas.context.set_font("bold 35px serif");
    let _ = canvas
        .context
        .fill_text_with_max_width(&text, x, y, canvas.canvas.width().into());
    // canvas.context.rect(0.0, 0.0, 400.0, 400.0);

    canvas.context.restore();
}

pub fn clear_previous_frame(id: String, col_index: i64, to_row: i64) {
    let canvas = Canvas::new(id);
    for y in 0..to_row + 1 {
        canvas
            .context
            .clear_rect((col_index * 75 + 50) as f64, (y * 75) as f64, 100.0, 100.0);
    }
}

pub fn redraw_col_mask(id: String, col_index: i64, to_row: i64) {
    let canvas = Canvas::new(id);

    let fill_value = "#FA9884".to_string();
    canvas.context.set_fill_style(&fill_value.into());
    canvas.context.begin_path();

    for y in 0..to_row + 1 {
        let x_f = (75 * col_index + 100) as f64;
        let y_f = (75 * y + 50) as f64;
        // r 25
        let _ = canvas.context.arc(x_f, y_f, 25.0, 0.0, 2.0 * PI);
        canvas.context.rect(x_f + 50.0, y_f - 50.0, -100.0, 100.0);
    }
    canvas.context.fill();
    canvas.context.restore();
}

pub fn animate(
    canvas_id: String,
    column: i64,
    to_row: i64,
    current_position: i64,
    fill_value: String,
    text: Option<String>,
    is_win: bool,
    is_draw: bool,
    current_player_name: String,
) {
    let window = web_sys::window().unwrap();

    if to_row * 75 >= current_position {
        log!("draw circle");
        if current_position > 0 {
            clear_previous_frame(canvas_id.clone(), column, to_row);
            redraw_col_mask(canvas_id.clone(), column, to_row);
        }
        draw_circle_at_canvas(
            canvas_id.clone(),
            fill_value.clone(),
            (75 * column + 100) as f64,
            (current_position + 50) as f64,
            text.clone(),
        );
        if to_row * 75 == current_position {
            log!("else in animation");
            if is_win {
                let win_string = format!(
                    "{} wins! clicked board to restart",
                    current_player_name.to_string()
                );
                display_text_on_canvas(canvas_id.clone(), win_string, "black".to_string());
                // display_text_on_canvas(win_string);
            } else {
                if is_draw {
                    display_text_on_canvas(
                        canvas_id.clone(),
                        "draw! click on board to restart the game".to_string(),
                        "black".to_owned(),
                    )
                }
            }
        }

        let closure = Closure::wrap(Box::new(move || {
            animate(
                canvas_id.clone(),
                column,
                to_row,
                current_position + 25,
                fill_value.clone(),
                text.clone(),
                is_win,
                is_draw,
                current_player_name.clone(),
            )
        }) as Box<dyn FnMut()>);

        let _ = window.request_animation_frame(closure.as_ref().unchecked_ref());
        closure.forget();
    }
}
