//! Submodule to initialize the `reagents` in the database.

pub mod distilled_water;
pub mod ethanol;
pub mod formic_acid;
pub mod liquid_nitrogen;
pub mod methanol;
pub use distilled_water::distilled_water;
pub use ethanol::absolute_ethanol;
pub use formic_acid::formic_acid;
pub use liquid_nitrogen::liquid_nitrogen;
pub use methanol::methanol_hplc;
