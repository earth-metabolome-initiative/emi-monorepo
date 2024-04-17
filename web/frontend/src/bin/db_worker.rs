use frontend::workers::*;
use yew_agent::Registrable;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    DBWorker::registrar().register();
}
