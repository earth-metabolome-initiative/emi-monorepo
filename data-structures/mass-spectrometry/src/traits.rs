//! Submodule defining traits for Mass Spectrometry

pub mod reference_spectra;
pub mod spectra;
pub mod spectral_filter;
pub mod spectral_pipeline;
pub mod spectral_pipeline_builder;
pub mod spectral_processor;
pub mod spectral_similarity;
pub mod spectrum;
pub mod spectrum_annotation;
pub mod spectrum_mut;

pub use reference_spectra::{
    AspirinSpectrum, CocaineSpectrum, GlucoseSpectrum, HydroxyCholesterolSpectrum,
    PhenylalanineSpectrum, SalicinSpectrum, EpimeloscineSpectrum
};
pub use spectra::Spectra;
pub use spectral_filter::SpectralFilter;
pub use spectral_pipeline::SpectralPipeline;
pub use spectral_pipeline_builder::SpectralPipelineBuilder;
pub use spectral_processor::SpectralProcessor;
pub use spectral_similarity::ScalarSpectralSimilarity;
pub use spectrum::Spectrum;
pub use spectrum_annotation::Annotation;
pub use spectrum_mut::{SpectrumAlloc, SpectrumMut};
