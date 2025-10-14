//! Implementation of the `FunctionLike` trait for sqlparser's `CreateFunction` type.

use sqlparser::ast::{CreateFunction, ObjectNamePart};

use crate::traits::FunctionLike;

impl FunctionLike for CreateFunction {
	fn name(&self) -> &str {
		match self.name.0.last() {
			Some(ObjectNamePart::Identifier(ident)) => &ident.value,
			Some(ObjectNamePart::Function(ident)) => &ident.name.value,
			None => unreachable!("Function name should not be empty"),
		}
	}
}