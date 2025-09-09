//! Submodule defining rules that all tables in the database must satisfy.

use diesel::PgConnection;
use procedure_codegen::constraints::{
    ProcedureAlignmentConstraint, ProcedurePrimaryKeyConstraint,
    ProcedureTemplatePrimaryKeyConstraint, ProcedureToProcedureTemplateForeignKeyConstraint,
    UnusedForeignProcedureTemplateConstraint,
};
use time_requirements::{prelude::TimeTracker, task::Task};
use webcodegen::{
    CompatibleForeignTypeConstraint, CustomColumnConstraint, CustomTableConstraint,
    HasSpecificTypeConstraint, LowercaseColumnConstraint, LowercaseTableConstraint,
    NotNullColumnConstraint, WordDeprecationConstraint,
};

pub(crate) fn execute_consistency_constraint_checks(
    database_name: &str,
    conn: &mut PgConnection,
) -> Result<TimeTracker, crate::errors::Error> {
    let mut time_tracker = TimeTracker::new("Consistency Constraint Checks");
    let schema = "public";

    let task = Task::new("Procedure and procedure template alignment");
    ProcedureAlignmentConstraint.check_all(database_name, schema, conn)?;
    time_tracker.add_completed_task(task);

    let mut sub_time_tracker = TimeTracker::new(&format!("Check constraints in schema '{schema}'"));
    let task = Task::new("Compatible foreign type constraints");
    CompatibleForeignTypeConstraint.check_all(database_name, schema, conn)?;
    sub_time_tracker.add_completed_task(task);
    let task = Task::new("Lowercase column and table names");
    LowercaseColumnConstraint.check_all(database_name, schema, conn)?;
    LowercaseTableConstraint.check_all(database_name, schema, conn)?;
    sub_time_tracker.add_completed_task(task);
    let task = Task::new("Standard column names and types");
    HasSpecificTypeConstraint::new("created_by", "integer").check_all(
        database_name,
        schema,
        conn,
    )?;
    HasSpecificTypeConstraint::new("updated_by", "integer").check_all(
        database_name,
        schema,
        conn,
    )?;
    HasSpecificTypeConstraint::new("created_at", "timestamp with time zone").check_all(
        database_name,
        schema,
        conn,
    )?;
    HasSpecificTypeConstraint::new("updated_at", "timestamp with time zone").check_all(
        database_name,
        schema,
        conn,
    )?;
    HasSpecificTypeConstraint::new("qrcode", "uuid").check_all(database_name, schema, conn)?;
    HasSpecificTypeConstraint::new("geolocation", "geography").check_all(
        database_name,
        schema,
        conn,
    )?;
    sub_time_tracker.add_completed_task(task);
    let task = Task::new("Not-null constraints on standard columns");
    NotNullColumnConstraint::new("created_by").check_all(database_name, schema, conn)?;
    NotNullColumnConstraint::new("updated_by").check_all(database_name, schema, conn)?;
    NotNullColumnConstraint::new("created_at").check_all(database_name, schema, conn)?;
    NotNullColumnConstraint::new("updated_at").check_all(database_name, schema, conn)?;
    sub_time_tracker.add_completed_task(task);
    let task = Task::new("Word deprecation constraints");
    let constraint = WordDeprecationConstraint::from(vec!["trackable"]);
    <WordDeprecationConstraint as CustomColumnConstraint>::check_all(
        &constraint,
        database_name,
        schema,
        conn,
    )?;
    <WordDeprecationConstraint as CustomTableConstraint>::check_all(
        &constraint,
        database_name,
        schema,
        conn,
    )?;
    sub_time_tracker.add_completed_task(task);
    time_tracker.extend(sub_time_tracker);

    let task = Task::new("Procedure and procedure template check constraints");
    ProcedurePrimaryKeyConstraint.check_all(database_name, schema, conn)?;
    ProcedureTemplatePrimaryKeyConstraint.check_all(database_name, schema, conn)?;
    ProcedureToProcedureTemplateForeignKeyConstraint.check_all(database_name, schema, conn)?;
    UnusedForeignProcedureTemplateConstraint.check_all(database_name, schema, conn)?;
    time_tracker.add_completed_task(task);

    // TODO!: All textual fields in all tables that are not CSVs should have a check
    // constraint to validate that the field is correct within their own context
    // (e.g. not empty!)

    Ok(time_tracker)
}
