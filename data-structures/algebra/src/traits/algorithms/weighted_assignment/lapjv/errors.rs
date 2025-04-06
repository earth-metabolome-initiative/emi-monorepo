//! Submodule providing the errors enumeration for the LAPJV algorithm.

#[derive(Debug, Clone, PartialEq)]
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