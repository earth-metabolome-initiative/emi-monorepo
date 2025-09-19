//! Implementation of the [`Translator`] trait for the
//! [`Function`](sqlparser::ast::Function) type.

use sqlparser::ast::{Function, Ident, ObjectName};

use crate::prelude::{Pg2SqliteOptions, PgSchema, Translator};

fn translate_function_name(name: &ObjectName) -> ObjectName {
    let original_name = name.to_string();
    let translated_name = match original_name.as_str() {
        "LEAST" => "MIN",
        "GREATEST" => "MAX",
        other => other,
    };
    ObjectName::from(vec![Ident::new(translated_name.to_string())])
}

impl Translator for Function {
    type Schema = PgSchema;
    type Options = Pg2SqliteOptions;
    type SQLiteEntry = Self;

    fn translate(
        &self,
        _schema: &mut Self::Schema,
        _options: &Self::Options,
    ) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        Ok(Function {
            name: translate_function_name(&self.name),
            uses_odbc_syntax: self.uses_odbc_syntax,
            parameters: self.parameters.clone(),
            args: self.args.clone(),
            filter: self.filter.clone(),
            null_treatment: self.null_treatment,
            over: self.over.clone(),
            within_group: self.within_group.clone(),
        })
    }
}
