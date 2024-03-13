use frontend::components::App;

fn main() {
    // We initialize the logger for the frontend
    wasm_logger::init(wasm_logger::Config::default());
    // And then we render the App component
    yew::Renderer::<App>::new().render();
}
