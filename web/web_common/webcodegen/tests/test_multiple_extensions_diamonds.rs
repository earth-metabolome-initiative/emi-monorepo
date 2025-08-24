//! Submodule testing the detection of columns requiring a partial builder.

mod utils;

use diesel::PgConnection;
use diesel_migrations_utils::prelude::MigrationDirectory;
use utils::*;
use webcodegen::{errors::WebCodeGenError, *};

fn test_diamond_methods(
    conn: &mut PgConnection,
    database_name: &str,
) -> Result<(), WebCodeGenError> {
    let trackables = Table::load(conn, "trackables", None, &database_name)?;
    assert!(trackables.extension_tables(conn)?.is_empty());
    let containers = Table::load(conn, "containers", None, &database_name)?;
    assert_eq!(containers.extension_tables(conn)?, vec![trackables.clone()]);
    let instruments = Table::load(conn, "instruments", None, &database_name)?;
    assert_eq!(instruments.extension_tables(conn)?, vec![trackables.clone()]);
    let freezers = Table::load(conn, "freezers", None, &database_name)?;
    assert_eq!(freezers.extension_tables(conn)?, vec![containers.clone(), instruments.clone()]);
    let android_devices = Table::load(conn, "android_devices", None, &database_name)?;
    assert_eq!(android_devices.extension_tables(conn)?, vec![instruments.clone()]);
    let android_freezers = Table::load(conn, "android_freezers", None, &database_name)?;
    assert_eq!(
        android_freezers.extension_tables(conn)?,
        vec![freezers.clone(), android_devices.clone()]
    );
    let gps = Table::load(conn, "gps", None, &database_name)?;
    assert_eq!(gps.extension_tables(conn)?, vec![instruments.clone()]);
    let cameras = Table::load(conn, "cameras", None, &database_name)?;
    assert_eq!(cameras.extension_tables(conn)?, vec![instruments.clone()]);
    let video_cameras = Table::load(conn, "video_cameras", None, &database_name)?;
    assert_eq!(video_cameras.extension_tables(conn)?, vec![cameras.clone()]);
    let phones = Table::load(conn, "phones", None, &database_name)?;
    assert_eq!(phones.extension_tables(conn)?, vec![gps.clone(), video_cameras.clone()]);
    let android_phones = Table::load(conn, "android_phones", None, &database_name)?;
    assert_eq!(
        android_phones.extension_tables(conn)?,
        vec![android_devices.clone(), phones.clone()]
    );

    let network = TableExtensionNetwork::new(conn, &database_name)?;
    let dot = network.to_dot();
    std::fs::write("test_multiple_extensions_diamonds.dot", dot).expect("Failed to write DOT file");

    for expected_bottom in [&android_freezers, &android_phones, &freezers] {
        assert!(network.is_diamond_bottom(expected_bottom).is_some());
    }

    for expected_top in [&trackables, &instruments] {
        assert!(network.is_diamond_top(expected_top));
    }

    for expected_side in [&containers, &gps, &cameras, &android_devices, &instruments, &freezers] {
        assert!(network.is_diamond_side(expected_side));
    }

    Ok(())
}

#[tokio::test]
/// Test retrieval of extensions from a column
async fn test_multiple_exensions_diamonds() {
    let (docker, mut conn, database_name) = setup_database_with_migrations(
        "test_multiple_exensions_diamonds",
        MigrationDirectory::try_from("./test_multiple_exensions_diamonds").unwrap(),
    )
    .await
    .unwrap();

    let outcome = test_diamond_methods(&mut conn, &database_name);

    docker.stop().await.unwrap();
    outcome.expect("Failed to generate code for multiple extensions");
}
