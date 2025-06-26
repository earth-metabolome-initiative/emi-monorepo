//! Submodule testing whether the `must_be_inserted_alongside_with` method
//! works as expected.

mod utils;

use std::collections::HashMap;

use diesel::PgConnection;
use diesel_migrations_utils::prelude::MigrationDirectory;
use utils::*;
use webcodegen::{ColumnSameAsNetwork, Table, errors::WebCodeGenError};

fn test_inserted_alongside_method(
    conn: &mut PgConnection,
    database_name: &str,
) -> Result<(), WebCodeGenError> {
    let trackables = Table::load(conn, "trackables", None, database_name)
        .expect("Failed to load the trackables table");
    let procedure_models = Table::load(conn, "procedure_models", None, database_name)
        .expect("Failed to load the procedure_models table");
    let trackable_procedure_models =
        Table::load(conn, "trackable_procedure_models", None, database_name)
            .expect("Failed to load the trackable_procedure_models table");
    let weighing_procedure_models =
        Table::load(conn, "weighing_procedure_models", None, database_name)
            .expect("Failed to load the weighing_procedure_models table");
    let freezing_procedure_models =
        Table::load(conn, "freezing_procedure_models", None, database_name)
            .expect("Failed to load the freezing_procedure_models table");
    let weighing_devices = Table::load(conn, "weighing_devices", None, database_name)
        .expect("Failed to load the weighing_devices table");
    let specialized_weighing_procedure_models =
        Table::load(conn, "specialized_weighing_procedure_models", None, database_name)
            .expect("Failed to load the specialized_weighing_procedure_models table");

    let mut expected_outcomes: HashMap<(&Table, &Table), bool> = HashMap::new();
    expected_outcomes.insert((&weighing_procedure_models, &procedure_models), true);
    expected_outcomes.insert((&weighing_procedure_models, &trackable_procedure_models), true);
    expected_outcomes.insert((&freezing_procedure_models, &procedure_models), true);
    expected_outcomes.insert((&freezing_procedure_models, &trackable_procedure_models), true);
    expected_outcomes
        .insert((&specialized_weighing_procedure_models, &weighing_procedure_models), true);
    expected_outcomes
        .insert((&specialized_weighing_procedure_models, &trackable_procedure_models), true);
    expected_outcomes.insert((&specialized_weighing_procedure_models, &procedure_models), true);
    expected_outcomes.insert((&weighing_devices, &trackables), true);

    // We symmetrize the expected outcomes for easier comparison
    for (&(left, right), &value) in expected_outcomes.clone().iter() {
        expected_outcomes.insert((right, left), value);
    }

    for left in [
        &trackables,
        &procedure_models,
        &trackable_procedure_models,
        &weighing_procedure_models,
        &freezing_procedure_models,
        &weighing_devices,
        &specialized_weighing_procedure_models,
    ] {
        for right in [
            &trackables,
            &procedure_models,
            &trackable_procedure_models,
            &weighing_procedure_models,
            &freezing_procedure_models,
            &weighing_devices,
            &specialized_weighing_procedure_models,
        ] {
            if left == right {
                continue;
            }

            let expected_outcome = *expected_outcomes.get(&(left, right)).unwrap_or(&false);

            assert_eq!(
                left.must_be_inserted_alongside_with(right, conn)?,
                expected_outcome,
                "Method failed for `{}` and `{}`.",
                left.table_name,
                right.table_name
            );
        }
    }

    Ok(())
}

#[tokio::test]
/// Test retrieval of extensions from a column
async fn test_inserted_alongside() {
    let (docker, mut conn, database_name) = setup_database_with_migrations(
        "test_inserted_alongside",
        MigrationDirectory::try_from("./test_inserted_alongside").unwrap(),
    )
    .await
    .unwrap();

    let outcome = test_inserted_alongside_method(&mut conn, &database_name);
    let graph = ColumnSameAsNetwork::new(&mut conn, &database_name, None).unwrap();
    let to_dot = graph.to_dot(&mut conn).expect("Failed to convert the graph to dot format");
    std::fs::write("test_inserted_alongside.dot", to_dot)
        .expect("Failed to write the graph to a file");

    docker.stop().await.unwrap();
    outcome.expect("Failed to test the inserted alongside method");
}
