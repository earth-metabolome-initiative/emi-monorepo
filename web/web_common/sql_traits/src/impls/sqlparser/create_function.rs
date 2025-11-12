//! Implementation of the `FunctionLike` trait for sqlparser's `CreateFunction`
//! type.

use sqlparser::ast::{CreateFunction, ObjectNamePart};

use crate::{
    structs::ParserDB,
    traits::{FunctionLike, Metadata},
    utils::normalize_sqlparser_type,
};

impl Metadata for CreateFunction {
    type Meta = ();
}

impl FunctionLike for CreateFunction {
    type DB = ParserDB;

    #[inline]
    fn name(&self) -> &str {
        match self.name.0.last() {
            Some(ObjectNamePart::Identifier(ident)) => &ident.value,
            Some(ObjectNamePart::Function(ident)) => &ident.name.value,
            None => unreachable!("Function name should not be empty"),
        }
    }

    #[inline]
    fn argument_type_names<'db>(
        &'db self,
        _database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db str> {
        self.args
            .iter()
            .flat_map(|args| args.iter().map(|arg| normalize_sqlparser_type(&arg.data_type)))
    }

    #[inline]
    fn return_type_name<'db>(&'db self, _database: &'db Self::DB) -> Option<&'db str> {
        self.return_type.as_ref().map(normalize_sqlparser_type)
    }
}
