//! Submodule defining traits for Mass Spectrometry

pub mod spectra;
pub mod spectral_similarity;
pub mod spectrum;
pub mod spectrum_annotation;
pub mod spectral_filter;
pub mod spectral_pipeline;
pub mod spectral_pipeline_builder;
pub mod spectral_processor;

pub use spectra::Spectra;
pub use spectral_similarity::ScalarSpectralSimilarity;
pub use spectrum::Spectrum;
pub use spectrum_annotation::Annotation;
pub use spectral_filter::SpectralFilter;
pub use spectral_pipeline::SpectralPipeline;
pub use spectral_pipeline_builder::SpectralPipelineBuilder;
pub use spectral_processor::SpectralProcessor;
