#[cfg(target_arch = "wasm32")]
fn main() {
    use frontend::workers::WebsocketWorker;
    use yew_agent::Registrable;
    wasm_logger::init(wasm_logger::Config::default());

    WebsocketWorker::registrar().register();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    panic!("This binary is only for the wasm32 target");
}
