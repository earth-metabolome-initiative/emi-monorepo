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

    // We load the table "instrumented_procedure_models"
    let instrumented_procedure_models =
        Table::load(&mut conn, "instrumented_procedure_models", None, &database_name)
            .expect("Failed to load table `instrumented_procedure_models`.");

    let procedure_model_trackables =
        Table::load(&mut conn, "procedure_model_trackables", None, &database_name)
            .expect("Failed to load table `procedure_model_trackables`.");

    let instrument_id = instrumented_procedure_models
        .column_by_name(&mut conn, "instrument_id")
        .expect("Failed to find column `instrument_id` in table `instrumented_procedure_models`.");

    assert!(
        instrument_id
            .requires_partial_builder(&mut conn)
            .expect("Failed to check if column `instrument_id` requires a partial builder.")
            .is_some(),
        "Column `instrument_id` should require a partial builder."
    );

    let procedure_model_trackable_columns = procedure_model_trackables
        .columns(&mut conn)
        .expect("Failed to retrieve columns from table `procedure_model_trackables`.");

    for column in procedure_model_trackable_columns {
        assert!(
            column
                .requires_partial_builder(&mut conn)
                .expect("Failed to check if column `column` requires a partial builder.")
                .is_none(),
            "Column `column` should require a partial builder."
        );
    }

    // We now check that `specific_procedures`'s table column `procedure_model_id`
    // is correctly identified as `same-as` `procedures`'s `procedure_model_id`
    // column. This check is obtained via the existence of the `UNIQUE`
    // constraints which operates of the `id` and `procedure_model_id` columns.
    let specific_procedures = Table::load(&mut conn, "specific_procedures", None, &database_name)
        .expect("Failed to load table `specific_procedures`.");

    let procedure_model_id = specific_procedures
        .column_by_name(&mut conn, "procedure_model_id")
        .expect("Failed to find column `procedure_model_id` in table `specific_procedures`.");

    let mut procedure_model_id_same_as = procedure_model_id
        .same_as_columns(&mut conn)
        .expect("Failed to check if column `procedure_model_id` is same as another column.");

    assert_eq!(
        procedure_model_id_same_as.len(),
        1,
        "Column `procedure_model_id` should have exactly one `same-as` constraint."
    );

    let Some(same_as_column) = procedure_model_id_same_as.pop() else {
        panic!("Expected at least one `same-as` constraint for column `procedure_model_id`.");
    };

    let procedures = Table::load(&mut conn, "procedures", None, &database_name)
        .expect("Failed to load table `procedures`.");

    let procedures_procedure_model_id = procedures
        .column_by_name(&mut conn, "procedure_model_id")
        .expect("Failed to find column `procedure_model_id` in table `procedures`.");

    assert_eq!(
        same_as_column, procedures_procedure_model_id,
        "Column `procedure_model_id` in table `specific_procedures` should be the same as `procedure_model_id` in table `procedures`."
    );

    docker.stop().await.unwrap();
}
