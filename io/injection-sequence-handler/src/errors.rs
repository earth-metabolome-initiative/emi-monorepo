//! Error enumeration for the injection-sequence-handler

use std::fmt::Display;


#[derive(Debug, Clone)]
/// Error enumeration for the injection-sequence-handler
pub enum Error {
    /// An unknown color was provided
    UnknownRackColor(char),
    /// An unknown row letter was provided
    /// From A to F.
    UnknownRowLetter(char),
    /// An unknown column number was provided
    /// From 1 to 12.
    UnknownColumnNumber(char),
    /// An unknown injection position was provided
    /// Something like Y:A1
    UnknownInjectionPosition(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnknownRackColor(unknown_rack_color   ) => write!(f, "Unknown rack color {unknown_rack_color} was provided"),
            Error::UnknownRowLetter(unknown_row_letter) => write!(f, "Unknown row letter {unknown_row_letter} was provided"),
            Error::UnknownColumnNumber(unknown_column_number) => write!(f, "Unknown column number {unknown_column_number} was provided"),
            Error::UnknownInjectionPosition(unknown_injection_position) => write!(f, "Unknown injection position {unknown_injection_position} was provided"),
        }
    }
}

impl std::error::Error for Error {}