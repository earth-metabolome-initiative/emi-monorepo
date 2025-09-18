use sqlparser::ast::{ConditionalStatements, CreateTrigger, DropTrigger, TriggerExecBodyType};

use crate::{
    options::Pg2SqliteOptions,
    prelude::PgSchema,
    traits::{schema::Schema, translator::Translator},
};

impl Translator for CreateTrigger {
    type Schema = PgSchema;
    type Options = Pg2SqliteOptions;
    type SQLiteEntry = (Option<DropTrigger>, Self);

    fn translate(
        &self,
        schema: &mut Self::Schema,
        options: &Self::Options,
    ) -> Result<Self::SQLiteEntry, crate::errors::Error> {
        let CreateTrigger {
            or_alter,
            temporary,
            or_replace,
            is_constraint,
            name,
            period_specified_before_table,
            period,
            events,
            table_name,
            referenced_table_name,
            referencing,
            trigger_object,
            include_each,
            condition,
            statements_as,
            exec_body,
            statements,
            characteristics,
        } = self.clone();

        if let Some(statements) = statements {
            return Err(crate::errors::Error::UnknownPostgresFeature(
                format!("Triggers with statements are not supported: `{}`", statements).into(),
            ));
        }

        let Some(exec_body) = exec_body else {
            return Err(crate::errors::Error::UnknownPostgresFeature(
                "Triggers without an execution body are not supported".into(),
            ));
        };

        if matches!(exec_body.exec_type, TriggerExecBodyType::Procedure) {
            return Err(crate::errors::Error::UnknownPostgresFeature(
                format!(
                    "Triggers with execution body of type `Procedure` are not supported: `{}`",
                    exec_body
                )
                .into(),
            ));
        }

        let function_name = exec_body.func_desc.name;
        let function_body = schema.function_body(&function_name.to_string()).ok_or_else(|| {
            crate::errors::Error::UnknownPostgresFeature(
                format!("Trigger function `{}` is not defined", function_name).into(),
            )
        })?;

        let maybe_drop_trigger = or_replace.then(|| {
            DropTrigger {
                if_exists: true,
                trigger_name: name.clone(),
                table_name: None,
                option: None,
            }
        });

        if or_alter {
            return Err(crate::errors::Error::UnknownPostgresFeature(
                "Triggers with `OR ALTER` are not supported".into(),
            ));
        }

        if is_constraint {
            return Err(crate::errors::Error::UnknownPostgresFeature(
                "Constraint triggers are not supported".into(),
            ));
        }

        if let Some(characteristics) = &characteristics {
            return Err(crate::errors::Error::UnknownPostgresFeature(
                format!("Triggers with characteristics are not supported: `{}`", characteristics)
                    .into(),
            ));
        }

        Ok((
            maybe_drop_trigger,
            CreateTrigger {
                or_alter,
                temporary,
                or_replace: false,
                is_constraint,
                name,
                period_specified_before_table,
                period,
                events,
                table_name,
                referenced_table_name,
                referencing,
                trigger_object,
                include_each,
                statements_as,
                condition: condition
                    .as_ref()
                    .map(|cond| cond.translate(schema, options))
                    .transpose()?,
                exec_body: None,
                statements: Some(ConditionalStatements::BeginEnd(function_body)),
                characteristics: None,
            },
        ))
    }
}
