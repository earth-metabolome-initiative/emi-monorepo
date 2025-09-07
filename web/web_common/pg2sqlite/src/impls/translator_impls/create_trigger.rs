//! Function providing the translation for a CreateTrigger variant of a
//! statement from the sqlparser crate. This variant of a statement does not
//! have a struct associated to it, therefore we cannot implement the Translator
//! trait for it directly. Instead, we provide a function that can be used in
//! the match statement in the translate function of the Statement
//! implementation of the Translator trait.

use sqlparser::ast::{
    ConditionalStatements, ConstraintCharacteristics, Expr, ObjectName, Statement, TriggerEvent,
    TriggerExecBody, TriggerObject, TriggerPeriod, TriggerReferencing,
};

use crate::{options::Pg2SqliteOptions, prelude::PgSchema, traits::translator::Translator};

pub(super) fn translate_create_trigger(
    or_alter: bool,
    or_replace: bool,
    is_constraint: bool,
    name: &ObjectName,
    period: TriggerPeriod,
    events: &Vec<TriggerEvent>,
    table_name: &ObjectName,
    referenced_table_name: &Option<ObjectName>,
    referencing: &Vec<TriggerReferencing>,
    trigger_object: TriggerObject,
    include_each: bool,
    condition: &Option<Expr>,
    exec_body: &Option<TriggerExecBody>,
    statements: &Option<ConditionalStatements>,
    characteristics: &Option<ConstraintCharacteristics>,
    schema: &mut PgSchema,
    options: &Pg2SqliteOptions,
) -> Result<Statement, crate::errors::Error> {
    Ok(Statement::CreateTrigger {
        or_alter,
        or_replace,
        is_constraint,
        name: name.clone(),
        period,
        events: events.clone(),
        table_name: table_name.clone(),
        referenced_table_name: referenced_table_name.clone(),
        referencing: referencing.clone(),
        trigger_object,
        include_each,
        condition: condition.as_ref().map(|cond| cond.translate(schema, options)).transpose()?,
        exec_body: exec_body.as_ref().map(|body| body.translate(schema, options)).transpose()?,
        statements: statements
            .as_ref()
            .map(|stmts| stmts.translate(schema, options))
            .transpose()?,
        characteristics: characteristics
            .as_ref()
            .map(|chars| chars.translate(schema, options))
            .transpose()?,
    })
}
