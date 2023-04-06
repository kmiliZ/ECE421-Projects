use web::App;
fn main() {
    // wasm_logger::init(wasm_logger::Config::default());
    // log::trace!("Initializing yew...");
    yew::start_app::<App>();
}
