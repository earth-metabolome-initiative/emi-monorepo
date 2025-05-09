//! Web-worker for web-socket communication.

// #[cfg(target_arch = "wasm32")]
/// Main function for the web-socket worker.
fn main() {
    use frontend::workers::DBWSWorker;
    use yew_agent::Registrable;
    wasm_logger::init(wasm_logger::Config::default());

    DBWSWorker::registrar().register();
}
