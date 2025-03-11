//! Submodule providing reference spectra for common molecules.

pub mod cocaine;
pub mod glucose;
pub mod hydroxycholesterol;
pub mod phenylalanine;

pub use cocaine::CocaineSpectrum;
pub use glucose::GlucoseSpectrum;
pub use hydroxycholesterol::HydroxyCholesterolSpectrum;
pub use phenylalanine::PhenylalanineSpectrum;