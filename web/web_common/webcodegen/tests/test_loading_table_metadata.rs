use diesel::pg::PgConnection;
use diesel::{Connection, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    ContainerAsync, GenericImage, ImageExt,
};
use webcodegen::*;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./test_migrations");
const DATABASE_NAME: &str = "test_db";
const DATABASE_PASSWORD: &str = "password";
const DATABASE_USER: &str = "user";
const DATABASE_PORT: u16 = 33676;

fn establish_connection_to_postres() -> PgConnection {
    let database_url = format!(
        "postgres://{DATABASE_USER}:{DATABASE_PASSWORD}@localhost:{DATABASE_PORT}/{DATABASE_NAME}",
    );

    let mut number_of_attempts = 0;

    while let Err(e) = PgConnection::establish(&database_url) {
        eprintln!("Failed to establish connection: {:?}", e);
        std::thread::sleep(std::time::Duration::from_secs(1));
        if number_of_attempts > 10 {
            eprintln!("Failed to establish connection after 10 attempts");
            std::process::exit(1);
        }
        number_of_attempts += 1;
    }

    PgConnection::establish(&database_url).unwrap()
}

fn teardown_test_database(conn: &mut PgConnection) {
    diesel::sql_query(format!("DROP DATABASE IF EXISTS {}", DATABASE_NAME))
        .execute(conn)
        .unwrap();
}

async fn setup_postgres() -> ContainerAsync<GenericImage> {
    let container = GenericImage::new("postgres", "17-alpine")
        .with_wait_for(WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        ))
        .with_network("bridge")
        .with_env_var("DEBUG", "1")
        .with_env_var("POSTGRES_USER", DATABASE_USER)
        .with_env_var("POSTGRES_PASSWORD", DATABASE_PASSWORD)
        .with_env_var("POSTGRES_DB", DATABASE_NAME)
        .with_mapped_port(DATABASE_PORT, 5432.tcp())
        .start()
        .await;

    if let Err(e) = container {
        eprintln!("Failed to start container: {:?}", e);
        std::process::exit(1);
    }

    container.unwrap()
}

#[tokio::test]
async fn test_user_table() {
    let container = setup_postgres().await;

    let mut conn = establish_connection_to_postres();
    conn.run_pending_migrations(MIGRATIONS).unwrap();

    // We try to load all elements of each type, so to ensure
    // that the structs are actually compatible with the schema
    // of PostgreSQL
    let all_tables = Table::load_all_tables(&mut conn);
    let all_columns = Column::load_all_columns(&mut conn);
    let all_table_constraints = TableConstraint::load_all_table_constraints(&mut conn);
    let all_key_column_usage = KeyColumnUsage::load_all_key_column_usages(&mut conn);
    let all_referential_constraints =
        ReferentialConstraint::load_all_referential_constraints(&mut conn);
    let all_constraint_column_usage =
        ConstraintColumnUsage::load_all_constraint_column_usages(&mut conn);
    let all_check_constraint = CheckConstraint::load_all_check_constraints(&mut conn);
    let all_domain_constraint = DomainConstraint::load_all_domain_constraints(&mut conn);

    let users = Table::load(&mut conn, "users", None, DATABASE_NAME).unwrap();

    let original_user_id_column = users.column_by_name(&mut conn, "id").unwrap();

    let columns: Result<Vec<Column>, diesel::result::Error> = users.columns(&mut conn);

    assert!(columns.is_ok());
    let columns = columns.unwrap();
    assert_eq!(columns.len(), 4);

    let primary_key_columns: Result<Vec<Column>, diesel::result::Error> =
        users.primary_key_columns(&mut conn);

    assert!(primary_key_columns.is_ok());
    let primary_key_columns = primary_key_columns.unwrap();
    assert_eq!(primary_key_columns.len(), 1);

    let unique_columns: Result<Vec<Vec<Column>>, diesel::result::Error> =
        users.unique_columns(&mut conn);

    assert!(unique_columns.is_ok());
    let unique_columns = unique_columns.unwrap();

    assert_eq!(unique_columns.len(), 3);
    assert_eq!(unique_columns[0].len(), 1);
    assert_eq!(unique_columns[1].len(), 2);
    assert_eq!(unique_columns[2].len(), 1);

    let composite_users = Table::load(&mut conn, "composite_users", None, DATABASE_NAME).unwrap();

    let columns: Result<Vec<Column>, diesel::result::Error> = composite_users.columns(&mut conn);
    let primary_key_columns: Result<Vec<Column>, diesel::result::Error> =
        composite_users.primary_key_columns(&mut conn);

    assert!(columns.is_ok());
    let columns = columns.unwrap();
    assert_eq!(columns.len(), 5);

    assert!(primary_key_columns.is_ok());
    let primary_key_columns = primary_key_columns.unwrap();
    assert_eq!(primary_key_columns.len(), 2);

    let primary_id_column = composite_users
        .column_by_name(&mut conn, "primary_id")
        .unwrap();
    assert_eq!(primary_id_column.column_name, "primary_id");
    assert!(primary_id_column.is_foreign_key(&mut conn));

    let (foreign_table, user_id_column) = primary_id_column.foreign_table(&mut conn).unwrap().unwrap();
    assert_eq!(foreign_table, users);
    assert_eq!(user_id_column, original_user_id_column);

    let secondary_id_column = composite_users
        .column_by_name(&mut conn, "secondary_id")
        .unwrap();
    assert_eq!(secondary_id_column.column_name, "secondary_id");
    assert!(secondary_id_column.is_foreign_key(&mut conn));

    let (foreign_table, user_id_column) = secondary_id_column
        .foreign_table(&mut conn)
        .unwrap()
        .unwrap();
    assert_eq!(foreign_table, users);
    assert_eq!(user_id_column, original_user_id_column);

    let username_column = composite_users
        .column_by_name(&mut conn, "username")
        .unwrap();
    assert_eq!(username_column.column_name, "username");
    assert!(!username_column.is_foreign_key(&mut conn));

    assert!(username_column.foreign_table(&mut conn).unwrap().is_none());

    container.stop().await.unwrap();
}
