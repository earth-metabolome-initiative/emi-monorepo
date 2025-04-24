//! Web-worker for processing JPEG files.

// #[cfg(target_arch = "wasm32")]
// ///
// fn main() {
//     use frontend::workers::FileProcessor;
//     use yew_agent::Registrable;
//     wasm_logger::init(wasm_logger::Config::default());

//     FileProcessor::<web_common::types::JPEG>::registrar().register();
// }

// #[cfg(not(target_arch = "wasm32"))]
/// Main function for the JPEG file processor worker.
fn main() {
    panic!("This binary is only for the wasm32 target");
}
