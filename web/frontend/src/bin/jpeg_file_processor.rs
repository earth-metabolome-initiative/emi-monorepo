use frontend::workers::FileProcessor;
use yew_agent::Registrable;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    FileProcessor::<web_common::types::JPEG>::registrar().register();
}
