mod api;
pub mod components;
mod cookies;
mod pages;
mod router;
mod stores;
mod utils;
pub mod workers;

#[cfg(target_arch = "wasm32")]
/// While we are always compiling for WASM32, I have yet to figure out how to
/// let the RustAnalyzer know that. So, I have to use cfg to make it happy.
/// Fortunately, instead, cargo check is aware of the target architecture.
mod database;
