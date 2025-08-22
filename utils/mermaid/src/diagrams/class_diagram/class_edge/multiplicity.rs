//! Submodule defining the multiplicities which may be used to characterize
//! a class edge in a class diagram in Mermaid syntax.
//!
//! The different cardinality options are :
//!
//! - `1` Only 1
//! - `0..1` Zero or One
//! - `1..*` One or more
//! - `*` Many
//! - `n` n (where n>1)
//! - `0..n` zero to n (where n>1)
//! - `1..n` one to n (where n>1)

use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// An enumeration representing the multiplicity of a class edge in a Mermaid
/// class diagram.
pub enum Multiplicity {
    /// Only 1
    One,
    /// Zero or One
    ZeroOrOne,
    /// One or more
    OneOrMore,
    /// Many, which is analogous to `Zero or More`
    Many,
    /// n (where n>1)
    N,
    /// Zero to n (where n>1)
    ZeroToN,
    /// One to n (where n>1)
    OneToN,
}

impl Display for Multiplicity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Multiplicity::One => write!(f, "1"),
            Multiplicity::ZeroOrOne => write!(f, "0..1"),
            Multiplicity::OneOrMore => write!(f, "1..*"),
            Multiplicity::Many => write!(f, "*"),
            Multiplicity::N => write!(f, "n"),
            Multiplicity::ZeroToN => write!(f, "0..n"),
            Multiplicity::OneToN => write!(f, "1..n"),
        }
    }
}
