//! Submodule providing an enumeration defining different types of durations for
//! Mermaid diagrams.

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents different types of durations that can be used in Mermaid
/// diagrams.
pub enum Duration {
    /// An infinite duration, meaning the animation will not end.
    Infinite,
    /// A specific duration in seconds.
    Seconds(u32),
}
