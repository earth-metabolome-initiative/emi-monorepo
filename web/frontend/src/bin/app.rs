#[cfg(target_arch = "wasm32")]
fn main() {
    use frontend::components::App;
    // We initialize the logger for the frontend
    wasm_logger::init(wasm_logger::Config::default());
    // And then we render the App component
    yew::Renderer::<App>::new().render();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    panic!("This binary is only for the wasm32 target");
}
