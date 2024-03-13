use yew_agent::Registrable;
use frontend::workers::WebsocketWorker;
use web_common::api::auth::ws::messages::{BackendMessage, FrontendMessage};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    WebsocketWorker::<FrontendMessage, BackendMessage>::registrar().register();
}