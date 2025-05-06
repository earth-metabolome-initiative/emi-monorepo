//! Code to embed the Postgres SQL files into the binary

#[cfg(feature = "pgrx")]
::pgrx::pgrx_embed!();

#[cfg(not(feature = "pgrx"))]
/// Stub function to allow compilation without pgrx
fn main() {
    println!("Hello, nameplate_categories!");
}
