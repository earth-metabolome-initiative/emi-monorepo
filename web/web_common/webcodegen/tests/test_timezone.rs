//! Submodule to test querying for the time zone of a database.

mod utils;

use utils::*;
use webcodegen::*;

#[tokio::test]
/// Test to query the time zone of a database
async fn test_timezone() {
    let (docker, mut conn, _database_name) =
        setup_database_with_default_migrations("test_timezone").await.unwrap();

    let time_zone = PgSetting::time_zone(&mut conn);

    docker.stop().await.unwrap();

    let time_zone: PgSetting = time_zone.expect("Failed to get time zone");
    assert_eq!(time_zone.setting, "UTC");
}
