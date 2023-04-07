use core::f64::consts::PI;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
// do canvas struct and implement draw_canvas, draw circle, animation and stuff
// let connect4 to have an option field for canvas_controller_object. whenver need to call canvas related,
// do canvas.unwrap? try that
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
        self.context.save();
        self.context.set_fill_style(&fill_value.into());
        self.context.begin_path();
        let x_f = (75 * x + 100) as f64;
        let y_f = (75 * y + 50) as f64;
        let _ = self.context.arc(x_f, y_f, r, 0.0, 2.0 * PI);
        self.context.fill();
        self.context.restore();
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

    pub fn fill_text(&self, text: String) {
        let bounding_rect = self.canvas.get_bounding_client_rect();
        let x = bounding_rect.width() / 2.0;
        let y = bounding_rect.height() / 2.0;
        self.context.begin_path();
        self.context.stroke();
        self.context.set_text_align("center");
        self.context.set_font("35px serif");
        let _ = self
            .context
            .fill_text_with_max_width(&text, x, y, self.canvas.width().into());
    }

    pub fn animate(&self) {
        let window = web_sys::window().unwrap();
        // window.request_animation_frame(callback);
        // TODO:fix this
    }
}
