//! Implementation of the `FunctionLike` trait for sqlparser's `CreateFunction` type.

use sqlparser::ast::{CreateFunction, ObjectNamePart};

use crate::{structs::ParserDB, traits::{FunctionLike, Metadata}};

impl Metadata for CreateFunction {
	type Meta = ();
}

impl FunctionLike for CreateFunction {
	type Database = ParserDB;

	fn name(&self) -> &str {
		match self.name.0.last() {
			Some(ObjectNamePart::Identifier(ident)) => &ident.value,
			Some(ObjectNamePart::Function(ident)) => &ident.name.value,
			None => unreachable!("Function name should not be empty"),
		}
	}

	fn argument_type_names(&self, _database: &Self::Database) -> Vec<String> {
		let Some(args_list) = &self.args else {
			return Vec::new();
		};
		let mut args = Vec::new();
		for arg in args_list {
			args.push(arg.data_type.to_string());
		}
		args
	}

	fn return_type_name(&self, _database: &Self::Database) -> Option<String> {
		self.return_type
			.as_ref()
			.map(|rt| rt.to_string())
	}
}
