//! Test suite for testing the `sibling_tables` method

mod utils;

use diesel_async::AsyncPgConnection;
use utils::*;
use webcodegen::{errors::WebCodeGenError, *};

async fn inner_test_sibling_tables(
    conn: &mut AsyncPgConnection,
    database_name: &str,
) -> Result<(), WebCodeGenError> {
    let teams = Table::load(conn, "teams", None, database_name).await?;
    let users = Table::load(conn, "users", None, database_name).await?;
    let projects = Table::load(conn, "projects", None, database_name).await?;
    let team_members = Table::load(conn, "team_members", None, database_name).await?;
    assert_eq!(team_members.parent_tables(conn).await?, vec![users.clone(), teams.clone()]);

    let team_projects = Table::load(conn, "team_projects", None, database_name).await?;
    assert_eq!(team_projects.parent_tables(conn).await?, vec![projects.clone(), teams.clone()]);

    let team_members_sibling = team_members.sibling_tables(conn).await?;
    assert!(team_members_sibling.contains(&team_projects));
    let team_projects_sibling = team_projects.sibling_tables(conn).await?;
    assert!(team_projects_sibling.contains(&team_members));

    Ok(())
}

#[tokio::test]
async fn test_sibling_tables() {
    let (docker, mut conn, database_name) =
        setup_database_with_default_migrations("test_sibling_tables").await.unwrap();

    let outcome = inner_test_sibling_tables(&mut conn, &database_name).await;
    docker.stop().await.unwrap();
    outcome.unwrap();
}
