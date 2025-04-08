//! Submodule providing the errors enumeration for the LAPJV algorithm.

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Errors that can occur during the execution of the LAPJV algorithm.
pub enum LAPJVError {
    /// The matrix is not square.
    NonSquareMatrix,
    /// The matrix is empty.
    EmptyMatrix,
    /// The matrix contains zero values.
    ZeroValues,
    /// The matrix contains negative values.
    NegativeValues,
    /// The matrix contains non-finite values.
    NonFiniteValues,
    /// The matrix contains a value larger than the maximum cost.
    ValueTooLarge,
    /// The provided maximal cost is not a finite number.
    MaximalCostNotFinite,
    /// The provided maximal cost is not a positive number.
    MaximalCostNotPositive,
    /// The provided padding value is not a finite number.
    PaddingValueNotFinite,
    /// The provided padding value is not a positive number.
    PaddingValueNotPositive,
}

impl Display for LAPJVError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            LAPJVError::NonSquareMatrix => write!(f, "The matrix is not square."),
            LAPJVError::EmptyMatrix => write!(f, "The matrix is empty."),
            LAPJVError::ZeroValues => write!(f, "The matrix contains zero values."),
            LAPJVError::NegativeValues => write!(f, "The matrix contains negative values."),
            LAPJVError::NonFiniteValues => write!(f, "The matrix contains non-finite values."),
            LAPJVError::ValueTooLarge => {
                write!(f, "The matrix contains a value larger than the maximum cost.")
            }
            LAPJVError::MaximalCostNotFinite => {
                write!(f, "The provided maximal cost is not a finite number.")
            }
            LAPJVError::MaximalCostNotPositive => {
                write!(f, "The provided maximal cost is not a positive number.")
            }
            LAPJVError::PaddingValueNotFinite => {
                write!(f, "The provided padding value is not a finite number.")
            }
            LAPJVError::PaddingValueNotPositive => {
                write!(f, "The provided padding value is not a positive number.")
            }
        }
    }
}

impl std::error::Error for LAPJVError {}
