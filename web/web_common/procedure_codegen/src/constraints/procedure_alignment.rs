//! Submodule providing a constraint to check that for each procedure template,
//! the associated procedure has all of the required columns.

use sqlparser::ast::CascadeOption;
use webcodegen::{ConstraintError, CustomTableConstraint, Table};

use crate::{
    PROCEDURE_ASSETS_TABLE_NAME, PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME, ProcedureTemplate,
};

/// Constraint to check that for each procedure template, the associated
/// procedure has all of the required columns.
pub struct ProcedureAlignmentConstraint;

impl CustomTableConstraint for ProcedureAlignmentConstraint {
    type Error = crate::errors::Error;

    fn check_constraint(
        &self,
        conn: &mut diesel::PgConnection,
        table: &webcodegen::Table,
    ) -> Result<(), Self::Error> {
        let Ok(procedure_template) = ProcedureTemplate::from_table(table.clone(), conn) else {
            // This table is not a procedure template table, so the constraint does not
            // apply.
            return Ok(());
        };

        // We check that the current procedure_template is not the abstract one but
        // a concrete one. Otherwise, there is no further check to do.
        if procedure_template.is_abstract() {
            return Ok(());
        }
        let procedure_template_asset_models_in_procedure_template =
            procedure_template.procedure_template_asset_models(conn)?;

        // For each of the procedure template asset models in the procedure template,
        // there must exist a unique index in the procedure template table which
        // includes the primary key of the procedure template and the procedure template
        // asset model column.
        let unique_indices = procedure_template.as_ref().unique_indices(conn)?;
        let unique_indices_columns = unique_indices
            .iter()
            .map(|index| index.columns(conn))
            .collect::<Result<Vec<_>, _>>()?;
        let procedure_template_primary_key = procedure_template.primary_key_column(conn)?;
        for (procedure_template_asset_model_column, _) in
            &procedure_template_asset_models_in_procedure_template
        {
            if procedure_template_asset_model_column.is_nullable() {
                return Err(ConstraintError::UnexpectedNullableColumn(Box::new(
                    procedure_template_asset_model_column.clone(),
                ))
                .into());
            }

            let expected_columns = vec![
                procedure_template_primary_key.clone(),
                procedure_template_asset_model_column.clone(),
            ];
            if !unique_indices_columns.iter().any(|cols| cols.as_ref() == &expected_columns) {
                return Err(ConstraintError::MissingUniqueIndex {
                    table: procedure_template.into(),
                    columns: expected_columns,
                }
                .into());
            }
        }

        let procedure = procedure_template.procedure(conn)?;
        let procedure_template_foreign_key =
            procedure.procedure_template_foreign_key(conn)?.unwrap();
        let procedure_template_column_in_procedure =
            procedure_template_foreign_key.columns(conn)?.as_ref()[0].clone();

        // For each procedure template asset model foreign key in the procedure
        // template, there must be a corresponding foreign key in the procedure.
        let procedure_template_asset_models_in_procedure =
            procedure.procedure_template_asset_models(conn)?;
        for (procedure_template_column, _) in &procedure_template_asset_models_in_procedure_template
        {
            if procedure_template_asset_models_in_procedure
                .iter()
                .find(|(procedure_column, _)| {
                    procedure_column.column_name == procedure_template_column.column_name
                })
                .is_some()
            {
                continue;
            }
            return Err(ConstraintError::MissingColumn {
                table: procedure.into(),
                column_name: procedure_template_column.column_name.clone(),
            }
            .into());
        }

        // For each procedure template asset model column in the procedure,
        // there must exist a foreign key which includes the primary key of the
        // procedure template and the procedure template asset model column
        // which points to the curresponding unique index in the procedure
        // template table.
        for (procedure_template_asset_model_in_procedure_template, _) in
            &procedure_template_asset_models_in_procedure_template
        {
            let (procedure_template_asset_model_in_procedure, _) =
                procedure_template_asset_models_in_procedure
                    .iter()
                    .find(|(procedure_column, _)| {
                        procedure_column.column_name
                            == procedure_template_asset_model_in_procedure_template.column_name
                    })
                    .unwrap();

            let expected_local_columns = vec![
                procedure_template_column_in_procedure.clone(),
                procedure_template_asset_model_in_procedure.clone(),
            ];
            let expected_foreign_columns = vec![
                procedure_template_primary_key.clone(),
                procedure_template_asset_model_in_procedure_template.clone(),
            ];

            let mut found_unique_index_foreign_key = false;
            for foreign_key in
                procedure_template_asset_model_in_procedure.foreign_keys(conn)?.iter()
            {
                let foreign_table = foreign_key.foreign_table(conn)?;
                if foreign_table.as_ref() != procedure_template.as_ref() {
                    continue;
                }
                let foreign_key_columns = foreign_key.columns(conn)?;
                let foreign_key_foreign_columns = foreign_key.foreign_columns(conn)?;
                if foreign_key_columns.as_ref() == &expected_local_columns
                    && foreign_key_foreign_columns.as_ref() == &expected_foreign_columns
                {
                    if foreign_key.has_on_delete_cascade(conn)? {
                        return Err(ConstraintError::ForeignKeyWithUnexpectedCascadingBehavior {
                            columns: expected_local_columns,
                            expected_behavior: CascadeOption::Restrict,
                            found_behavior: CascadeOption::Cascade,
                        }
                        .into());
                    }
                    found_unique_index_foreign_key = true;
                    break;
                }
            }

            if !found_unique_index_foreign_key {
                return Err(ConstraintError::MissingForeignKey {
                    columns: expected_local_columns,
                    referenced_columns: expected_foreign_columns,
                    cascade_option: CascadeOption::Restrict,
                }
                .into());
            }
        }

        let procedure_template_asset_models_table = Table::load(
            conn,
            PROCEDURE_TEMPLATE_ASSET_MODELS_TABLE_NAME,
            "public",
            &table.table_catalog,
        )?;
        let procedure_assets_table =
            Table::load(conn, PROCEDURE_ASSETS_TABLE_NAME, "public", &table.table_catalog)?;
        let procedure_template_asset_models_primary_key =
            procedure_template_asset_models_table.primary_key_columns(conn)?.as_ref()[0].clone();

        if procedure_template_asset_models_primary_key.is_nullable() {
            return Err(ConstraintError::UnexpectedNullableColumn(Box::new(
                procedure_template_asset_models_primary_key,
            ))
            .into());
        }

        let procedure_assets_primary_key =
            procedure_assets_table.primary_key_columns(conn)?.as_ref()[0].clone();

        if procedure_assets_primary_key.is_nullable() {
            return Err(ConstraintError::UnexpectedNullableColumn(Box::new(
                procedure_assets_primary_key,
            ))
            .into());
        }

        let asset_model_in_procedure_template_asset_models =
            procedure_template_asset_models_table.column_by_name(conn, "asset_model")?;

        if asset_model_in_procedure_template_asset_models.is_nullable() {
            return Err(ConstraintError::UnexpectedNullableColumn(Box::new(
                asset_model_in_procedure_template_asset_models,
            ))
            .into());
        }

        let asset_model_in_procedure_assets =
            procedure_assets_table.column_by_name(conn, "asset_model")?;

        if asset_model_in_procedure_assets.is_nullable() {
            return Err(ConstraintError::UnexpectedNullableColumn(Box::new(
                asset_model_in_procedure_assets,
            ))
            .into());
        }

        let asset_in_procedure_assets = procedure_assets_table.column_by_name(conn, "asset")?;
        let procedure_template_asset_model_in_procedure_assets_table =
            procedure_assets_table.column_by_name(conn, "procedure_template_asset_model")?;

        if procedure_template_asset_model_in_procedure_assets_table.is_nullable() {
            return Err(ConstraintError::UnexpectedNullableColumn(Box::new(
                procedure_template_asset_model_in_procedure_assets_table,
            ))
            .into());
        }

        let procedure_assets = procedure.procedure_assets(conn)?;

        // For each procedure template asset model foreign key in the procedure,
        // there must be a corresponding foreign key in the procedure assets.
        for (procedure_template_asset_model_in_procedure, _) in
            &procedure_template_asset_models_in_procedure
        {
            if procedure_template_asset_model_in_procedure.is_nullable() {
                return Err(ConstraintError::UnexpectedNullableColumn(Box::new(
                    procedure_template_asset_model_in_procedure.clone(),
                ))
                .into());
            }

            // If the procedure template asset models naming standard is:
            // - procedure_template_{asset_model_column}_model
            // then the corresponding procedure asset column should be:
            // - procedure_{asset_model_column}

            let expected_name = procedure_template_asset_model_in_procedure
                .column_name
                .as_str()
                .replacen("procedure_template_", "procedure_", 1)
                .strip_suffix("_model")
                .ok_or_else(|| {
                    let expected_name = format!(
                        "{}_model",
                        &procedure_template_asset_model_in_procedure.column_name
                    );
                    ConstraintError::DoesNotHaveExpectedName {
                        column: Box::new(procedure_template_asset_model_in_procedure.clone()),
                        expected_name,
                    }
                })?
                .to_owned();

            let Some((procedure_asset, _)) =
                procedure_assets.iter().find(|(procedure_asset_column, _)| {
                    procedure_asset_column.column_name == expected_name
                })
            else {
                return Err(ConstraintError::MissingColumn {
                    table: procedure.into(),
                    column_name: expected_name,
                }
                .into());
            };

            if procedure_asset.is_nullable() {
                return Err(ConstraintError::UnexpectedNullableColumn(Box::new(
                    procedure_asset.clone(),
                ))
                .into());
            }

            // The procedure asset column must have a foreign key pointing to the procedure
            // assets table which includes the associated
            // `procedure_template_asset_model_in_procedure` column to
            // ensure that the `procedure_template_asset_model_in_procedure` and the
            // `procedure_asset` are coupled and refer to the same asset model.
            let expected_local_columns =
                vec![procedure_asset.clone(), procedure_template_asset_model_in_procedure.clone()];
            let expected_foreign_columns = vec![
                procedure_assets_primary_key.clone(),
                procedure_template_asset_model_in_procedure_assets_table.clone(),
            ];
            let mut found_foreign_key = false;

            for foreign_key in procedure_asset.foreign_keys(conn)?.iter() {
                let foreign_table = foreign_key.foreign_table(conn)?;
                if foreign_table != procedure_assets_table {
                    continue;
                }
                let local_columns = foreign_key.columns(conn)?;
                let foreign_columns = foreign_key.foreign_columns(conn)?;
                if local_columns.as_ref() == &expected_local_columns
                    && foreign_columns.as_ref() == &expected_foreign_columns
                {
                    if foreign_key.has_on_delete_cascade(conn)? {
                        return Err(ConstraintError::ForeignKeyWithUnexpectedCascadingBehavior {
                            columns: expected_local_columns,
                            expected_behavior: CascadeOption::Restrict,
                            found_behavior: CascadeOption::Cascade,
                        }
                        .into());
                    }
                    found_foreign_key = true;
                    break;
                }
            }
            if !found_foreign_key {
                return Err(ConstraintError::MissingForeignKey {
                    columns: expected_local_columns,
                    referenced_columns: expected_foreign_columns,
                    cascade_option: CascadeOption::Restrict,
                }
                .into());
            }
        }

        // We load the asset models associated to the procedure template.
        let asset_models_in_procedure_template = procedure_template.asset_models(conn)?;
        // All asset models in the procedure template must be associated with the
        // curresponding procedure template asset model in the appropriate foreign key.
        // We can identify by name the corresponding procedure template asset model
        // column by the naming convention, where the procedure template asset
        // model column name is `procedure_template_{asset_model_column}`.
        for (asset_model_in_procedure_template, _) in &asset_models_in_procedure_template {
            if asset_model_in_procedure_template.is_nullable() {
                return Err(ConstraintError::UnexpectedNullableColumn(Box::new(
                    asset_model_in_procedure_template.clone(),
                ))
                .into());
            }

            let expected_name =
                format!("procedure_template_{}", asset_model_in_procedure_template.column_name);
            let (procedure_template_asset_model_in_procedure_template, _) =
                procedure_template_asset_models_in_procedure_template
                    .iter()
                    .find(|(procedure_template_asset_model_column, _)| {
                        procedure_template_asset_model_column.column_name == expected_name
                    })
                    .unwrap_or_else(|| panic!("Could not find procedure template asset model column with name \
                     {expected_name} associated with asset model column {expected_name}"));
            let mut found_foreign_key = false;
            let expected_local_columns = vec![
                procedure_template_asset_model_in_procedure_template.clone(),
                asset_model_in_procedure_template.clone(),
            ];
            let expected_foreign_columns = vec![
                procedure_template_asset_models_primary_key.clone(),
                asset_model_in_procedure_template_asset_models.clone(),
            ];
            for foreign_key in asset_model_in_procedure_template.foreign_keys(conn)?.as_ref() {
                let foreign_table = foreign_key.foreign_table(conn)?;
                if foreign_table != procedure_template_asset_models_table {
                    continue;
                }
                let local_columns = foreign_key.columns(conn)?;
                let foreign_columns = foreign_key.foreign_columns(conn)?;
                if local_columns.as_ref() == &expected_local_columns
                    && foreign_columns.as_ref() == &expected_foreign_columns
                {
                    if foreign_key.has_on_delete_cascade(conn)? {
                        return Err(ConstraintError::ForeignKeyWithUnexpectedCascadingBehavior {
                            columns: expected_local_columns,
                            expected_behavior: CascadeOption::Restrict,
                            found_behavior: CascadeOption::Cascade,
                        }
                        .into());
                    }
                    found_foreign_key = true;
                    break;
                }
            }
            if !found_foreign_key {
                return Err(ConstraintError::MissingForeignKey {
                    columns: expected_local_columns,
                    referenced_columns: expected_foreign_columns,
                    cascade_option: CascadeOption::Restrict,
                }
                .into());
            }
        }

        // We load the asset models associated to the procedure.
        let asset_models_in_procedure = procedure.asset_models(conn)?;

        // For each rule table, we check that all of the rules specified in the
        // procedure template table are also enforced in the procedure table.
        let procedure_template_rules = procedure_template.rules(conn)?;
        let procedure_rules = procedure.rules(conn)?;
        for procedure_template_rule in &procedure_template_rules {
            let procedure_template_asset_models = procedure_template_rule.columns(conn)?;
            let mut expected_procedure_asset_models = Vec::new();

            // We search the corresponding procedure asset models in the procedure
            // by name, and if we do not find them, we return an error.
            for procedure_template_asset_model in procedure_template_asset_models.iter() {
                let Some((procedure_asset_model, _)) =
                    asset_models_in_procedure.iter().find(|(procedure_asset_model, _)| {
                        procedure_asset_model.column_name
                            == procedure_template_asset_model.column_name
                    })
                else {
                    return Err(ConstraintError::MissingColumn {
                        table: procedure.into(),
                        column_name: procedure_template_asset_model.column_name.clone(),
                    }
                    .into());
                };

                expected_procedure_asset_models.push(procedure_asset_model.clone());
            }

            // We identify the rule in the procedure which corresponds to the
            // current procedure template rule by looking for a rule which has
            // the same asset models.
            let mut found_procedure_rule = false;
            for procedure_rule in &procedure_rules {
                let procedure_rule_asset_models = procedure_rule.columns(conn)?;
                if procedure_rule_asset_models.as_ref() != &expected_procedure_asset_models {
                    continue;
                }
                found_procedure_rule = true;
                break;
            }
            if !found_procedure_rule {
                return Err(ConstraintError::MissingForeignKey {
                    columns: expected_procedure_asset_models,
                    referenced_columns: procedure_template_rule
                        .foreign_columns(conn)?
                        .as_ref()
                        .clone(),
                    cascade_option: CascadeOption::Restrict,
                }
                .into());
            }
        }

        // Simmetrically, we check that there are no extra rules in the procedure
        // which are not present in the procedure template.
        for procedure_rule in &procedure_rules {
            let procedure_asset_models = procedure_rule.columns(conn)?;
            let mut expected_procedure_template_asset_models = Vec::new();

            // We search the corresponding procedure asset models in the procedure template
            // by name, and if we do not find them, we return an error.
            for procedure_asset_model in procedure_asset_models.iter() {
                let Some((procedure_template_asset_model, _)) = asset_models_in_procedure_template
                    .iter()
                    .find(|(procedure_template_asset_model, _)| {
                        procedure_template_asset_model.column_name
                            == procedure_asset_model.column_name
                    })
                else {
                    return Err(ConstraintError::UnexpectedColumn(Box::new(
                        procedure_asset_model.clone(),
                    ))
                    .into());
                };

                expected_procedure_template_asset_models
                    .push(procedure_template_asset_model.clone());
            }

            // We identify the rule in the procedure template which corresponds to the
            // current procedure rule by looking for a rule which has
            // the same asset models.
            let mut found_procedure_template_rule = false;
            for procedure_template_rule in &procedure_template_rules {
                let procedure_template_rule_asset_models = procedure_template_rule.columns(conn)?;
                if procedure_template_rule_asset_models.as_ref()
                    != &expected_procedure_template_asset_models
                {
                    continue;
                }
                found_procedure_template_rule = true;
                break;
            }
            if !found_procedure_template_rule {
                return Err(ConstraintError::UnexpectedForeignKey {
                    columns: procedure_asset_models.as_ref().clone(),
                    referenced_columns: procedure_rule.foreign_columns(conn)?.as_ref().clone(),
                }
                .into());
            }
        }

        let asset_compatibility_rules_table =
            Table::load(conn, "asset_compatibility_rules", "public", &table.table_catalog)?;
        let container_compatibility_rules_table =
            Table::load(conn, "container_compatibility_rules", "public", &table.table_catalog)?;

        let rule_tables =
            vec![asset_compatibility_rules_table, container_compatibility_rules_table];

        // All asset models in the procedure must be associated with the
        // curresponding procedure asset in the appropriate foreign key.
        // We can identify by name the corresponding procedure asset
        // column by the naming convention, where the procedure asset
        // column name is `procedure_{asset_column}`.
        for (asset_model_in_procedure, _) in &asset_models_in_procedure {
            if asset_model_in_procedure.is_nullable() {
                return Err(ConstraintError::UnexpectedNullableColumn(Box::new(
                    asset_model_in_procedure.clone(),
                ))
                .into());
            }

            // Asset models are only allowed in procedures if they also appear in
            // foreign keys pointing to the compatibility rules tables.
            let mut appears_in_foreign_key_to_rule_table = false;
            for rule_table in &rule_tables {
                if asset_model_in_procedure
                    .is_foreign_primary_key_of_table(rule_table, conn)?
                    .is_some()
                {
                    appears_in_foreign_key_to_rule_table = true;
                    break;
                }
            }
            if !appears_in_foreign_key_to_rule_table {
                return Err(ConstraintError::UnexpectedColumn(Box::new(
                    asset_model_in_procedure.clone(),
                ))
                .into());
            }

            let expected_name = format!(
                "procedure_{}",
                asset_model_in_procedure.column_name.as_str().strip_suffix("_model").unwrap()
            );
            let (procedure_asset, _) = procedure_assets
                .iter()
                .find(|(procedure_asset_column, _)| {
                    procedure_asset_column.column_name == expected_name
                })
                .unwrap_or_else(|| panic!("Could not find procedure asset model column with name \
                     {expected_name} associated with asset model column {expected_name}"));
            let mut found_foreign_key = false;
            let expected_local_columns =
                vec![procedure_asset.clone(), asset_model_in_procedure.clone()];
            let expected_foreign_columns =
                vec![procedure_assets_primary_key.clone(), asset_model_in_procedure_assets.clone()];
            for foreign_key in asset_model_in_procedure.foreign_keys(conn)?.as_ref() {
                let foreign_table = foreign_key.foreign_table(conn)?;
                if foreign_table != procedure_assets_table {
                    continue;
                }
                let local_columns = foreign_key.columns(conn)?;
                let foreign_columns = foreign_key.foreign_columns(conn)?;
                if local_columns.as_ref() == &expected_local_columns
                    && foreign_columns.as_ref() == &expected_foreign_columns
                {
                    if foreign_key.has_on_delete_cascade(conn)? {
                        return Err(ConstraintError::ForeignKeyWithUnexpectedCascadingBehavior {
                            columns: expected_local_columns,
                            expected_behavior: CascadeOption::Restrict,
                            found_behavior: CascadeOption::Cascade,
                        }
                        .into());
                    }
                    found_foreign_key = true;
                    break;
                }
            }
            if !found_foreign_key {
                return Err(ConstraintError::MissingForeignKey {
                    columns: expected_local_columns,
                    referenced_columns: expected_foreign_columns,
                    cascade_option: CascadeOption::Restrict,
                }
                .into());
            }
        }

        // We load the assets associated to the procedure.
        let assets_in_procedure = procedure.assets(conn)?;
        // All assets in the procedure must be associated with the
        // curresponding procedure asset in the appropriate foreign key.
        // We can identify by name the corresponding procedure asset
        // column by the naming convention, where the procedure asset
        // column name is `procedure_{asset_column}`.
        for (asset_in_procedure, _) in &assets_in_procedure {
            let expected_name = format!("procedure_{}", asset_in_procedure.column_name.as_str());
            let (procedure_asset, _) = procedure_assets
                .iter()
                .find(|(procedure_asset_column, _)| {
                    procedure_asset_column.column_name == expected_name
                })
                .unwrap_or_else(|| panic!("Could not find procedure asset model column with name \
                     {expected_name} associated with asset model column {expected_name}"));
            let mut found_foreign_key = false;
            let expected_local_columns = vec![procedure_asset.clone(), asset_in_procedure.clone()];
            let expected_foreign_columns =
                vec![procedure_assets_primary_key.clone(), asset_in_procedure_assets.clone()];
            for foreign_key in asset_in_procedure.foreign_keys(conn)?.iter() {
                let foreign_table = foreign_key.foreign_table(conn)?;
                if foreign_table != procedure_assets_table {
                    continue;
                }
                let local_columns = foreign_key.columns(conn)?;
                let foreign_columns = foreign_key.foreign_columns(conn)?;
                if local_columns.as_ref() == &expected_local_columns
                    && foreign_columns.as_ref() == &expected_foreign_columns
                {
                    if foreign_key.has_on_delete_cascade(conn)? {
                        return Err(ConstraintError::ForeignKeyWithUnexpectedCascadingBehavior {
                            columns: expected_local_columns,
                            expected_behavior: CascadeOption::Restrict,
                            found_behavior: CascadeOption::Cascade,
                        }
                        .into());
                    }
                    found_foreign_key = true;
                    break;
                }
            }
            if !found_foreign_key {
                return Err(ConstraintError::MissingForeignKey {
                    columns: expected_local_columns,
                    referenced_columns: expected_foreign_columns,
                    cascade_option: CascadeOption::Restrict,
                }
                .into());
            }
        }

        Ok(())
    }
}
