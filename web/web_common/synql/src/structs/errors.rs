//! Submodule providing error types used in SynQL.

/// Enum representing errors that can occur in SynQL.
#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Column type not found for column `{column_name}` in table `{table_name}` with SQL type `{sql_type}`")]
	/// Unknown column type error.
	ColumnTypeNotFound {
		/// Name of the table where the error occurred.
		table_name: String,
		/// Name of the column whose type was not found.
		column_name: String,
		/// SQL type of the column.
		sql_type: String,
	},
	#[error("Function definition not found for function `{function_name}`")]
	/// Function definition not found error.
	FunctionNotFound {
		/// Name of the function whose definition was not found.
		function_name: String,
	},
	#[error("I/O error: {0}")]
	/// I/O error.
	IO(#[from] std::io::Error),
}