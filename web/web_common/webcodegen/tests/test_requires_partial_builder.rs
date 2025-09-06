//! Submodule testing the detection of columns requiring a partial builder.

mod utils;

use diesel_migrations_utils::prelude::MigrationDirectory;
use utils::*;
use webcodegen::*;

#[tokio::test]
/// Test retrieval of extensions from a column
async fn test_requires_partial_builder() {
    let (docker, mut conn, database_name) = setup_database_with_migrations(
        "test_requires_partial_builder",
        MigrationDirectory::try_from("./test_requires_partial_builder").unwrap(),
    )
    .await
    .unwrap();

    // We load the table "instrumented_procedure_templates"
    let instrumented_procedure_templates =
        Table::load(&mut conn, "instrumented_procedure_templates", "public", &database_name)
            .expect("Failed to load table `instrumented_procedure_templates`.");

    let procedure_template_trackables =
        Table::load(&mut conn, "procedure_template_trackables", "public", &database_name)
            .expect("Failed to load table `procedure_template_trackables`.");

    let instrument_id =
        instrumented_procedure_templates.column_by_name(&mut conn, "instrument_id").expect(
            "Failed to find column `instrument_id` in table `instrumented_procedure_templates`.",
        );

    assert!(
        instrument_id
            .requires_partial_builder(&mut conn)
            .expect("Failed to check if column `instrument_id` requires a partial builder.")
            .is_some(),
        "Column `instrument_id` should require a partial builder."
    );

    let procedure_template_trackable_columns = procedure_template_trackables
        .columns(&mut conn)
        .expect("Failed to retrieve columns from table `procedure_template_trackables`.");

    for column in procedure_template_trackable_columns.iter() {
        assert!(
            column
                .requires_partial_builder(&mut conn)
                .expect("Failed to check if column `column` requires a partial builder.")
                .is_none(),
            "Column `column` should require a partial builder."
        );
    }

    docker.stop().await.unwrap();
}
