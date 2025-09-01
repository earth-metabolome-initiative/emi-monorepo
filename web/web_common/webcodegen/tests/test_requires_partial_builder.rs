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

    for column in procedure_template_trackable_columns {
        assert!(
            column
                .requires_partial_builder(&mut conn)
                .expect("Failed to check if column `column` requires a partial builder.")
                .is_none(),
            "Column `column` should require a partial builder."
        );
    }

    // We now check that `specific_procedures`'s table column `procedure_template`
    // is correctly identified as `same-as` `procedures`'s `procedure_template`
    // column. This check is obtained via the existence of the `UNIQUE`
    // constraints which operates of the `id` and `procedure_template` columns.
    let specific_procedures =
        Table::load(&mut conn, "specific_procedures", "public", &database_name)
            .expect("Failed to load table `specific_procedures`.");

    let procedure_template = specific_procedures
        .column_by_name(&mut conn, "procedure_template")
        .expect("Failed to find column `procedure_template` in table `specific_procedures`.");

    let mut procedure_template_same_as = procedure_template
        .same_as_columns(&mut conn)
        .expect("Failed to check if column `procedure_template` is same as another column.");

    assert_eq!(
        procedure_template_same_as.len(),
        1,
        "Column `procedure_template` should have exactly one `same-as` constraint."
    );

    let Some(same_as_column) = procedure_template_same_as.pop() else {
        panic!("Expected at least one `same-as` constraint for column `procedure_template`.");
    };

    let procedures = Table::load(&mut conn, "procedures", "public", &database_name)
        .expect("Failed to load table `procedures`.");

    let procedures_procedure_template = procedures
        .column_by_name(&mut conn, "procedure_template")
        .expect("Failed to find column `procedure_template` in table `procedures`.");

    assert_eq!(
        same_as_column, procedures_procedure_template,
        "Column `procedure_template` in table `specific_procedures` should be the same as `procedure_template` in table `procedures`."
    );

    docker.stop().await.unwrap();
}
