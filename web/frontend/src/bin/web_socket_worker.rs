use frontend::workers::WebsocketWorker;
use yew_agent::Registrable;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    WebsocketWorker::registrar().register();
}
