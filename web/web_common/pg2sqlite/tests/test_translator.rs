//! Test translating the core migrations used in the `core_structures` crate.

use diesel::{Connection, SqliteConnection, connection::SimpleConnection};
use pg2sqlite::{
    prelude::{Pg2Sqlite, Pg2SqliteOptions},
    traits::TranslationOptions,
};

#[test]
/// Test translating the core migrations used in the `core_structures` crate.
fn test_translator() {
    let to_parse = r#"CREATE TRIGGER trg_inherit_asset_models
AFTER INSERT ON parent_procedure_templates
FOR EACH ROW
BEGIN
    INSERT INTO procedure_template_asset_models (
        name,
        procedure_template,
        based_on,
        asset_model,
        created_by,
        created_at
    )
    SELECT
        pam.name,
        NEW.parent,
        pam.id,
        pam.asset_model,
        NEW.created_by,
        NEW.created_at
    FROM procedure_template_asset_models pam
    WHERE pam.procedure_template = NEW.child;
END;
"#;
    let stmt =
        sqlparser::parser::Parser::parse_sql(&sqlparser::dialect::SQLiteDialect {}, to_parse)
            .unwrap();
    println!("Parsed statement: {stmt:?}");

    let translated_migrations = Pg2Sqlite::default()
        .ups("../../../data_migrations/init_db/migrations")
        .expect("Failed to load the migrations")
        .translate(&Pg2SqliteOptions::default().remove_unsupported_check_constraints())
        .expect("Failed to translate the migrations");

    assert_eq!(translated_migrations.len(), 121);

    // We try to parse the translated migrations using the `sqlparser` crate,
    // for the `SQLite` dialect.
    for translated_migration in &translated_migrations {
        sqlparser::parser::Parser::parse_sql(
            &sqlparser::dialect::SQLiteDialect {},
            &translated_migration.to_string(),
        )
        .expect("Failed to parse the translated migration");
    }

    // We create a testcontainer `Docker` for `SQLite` and run the translated
    // migrations, considering the severe limitations of our target use case
    // which is `WASM + SQLite`.
    let mut connection = SqliteConnection::establish(":memory:")
        .expect("Failed to establish a connection to the SQLite database");

    let number_of_migrations = translated_migrations.len();
    for (i, translated_migration) in
        translated_migrations.iter().enumerate().map(|(v, s)| (v + 1, s))
    {
        let sql = translated_migration.to_string();
        if let Err(err) = connection.batch_execute(&sql) {
            panic!("Failed to run the translated statement {i}/{number_of_migrations} {sql}: {err}")
        }
    }
}
