//! Test suite for testing the `children_tables` method
mod utils;

use diesel_async::AsyncPgConnection;
use utils::*;
use webcodegen::{errors::WebCodeGenError, *};

async fn inner_test_children_tables(
    conn: &mut AsyncPgConnection,
    database_name: &str,
) -> Result<(), WebCodeGenError> {
    let teams = Table::load(conn, "teams", None, database_name).await?;
    let team_members = Table::load(conn, "team_members", None, database_name).await?;
    let team_projects = Table::load(conn, "team_projects", None, database_name).await?;

    let children = teams.children_tables(conn).await?;
    assert_eq!(children.len(), 2);
    assert_eq!(children, vec![team_members, team_projects]);

    Ok(())
}

#[tokio::test]
async fn test_children_tables() {
    let (docker, mut conn, database_name) =
        setup_database_with_default_migrations("test_children_tables").await.unwrap();

    let outcome = inner_test_children_tables(&mut conn, &database_name).await;
    docker.stop().await.unwrap();

    outcome.unwrap();
}
