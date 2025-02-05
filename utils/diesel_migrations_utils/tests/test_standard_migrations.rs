//! Test module for the standard migrations.
use std::path::Path;

use diesel_migrations_utils::prelude::*;

#[test]
/// Test to check that the standard migrations are correctly loaded.
pub fn test_standard_migrations() {
    let standard = MigrationDirectory::try_from("standard_migrations").unwrap();
    assert_eq!(standard.len(), 2);
    assert!(!standard.is_dense());
    let migrations = standard.migrations().collect::<Vec<&Migration>>();
    assert_eq!(migrations[0].number(), 1);
    assert_eq!(migrations[0].name(), "first");
    assert_eq!(migrations[0].directory(), "00000000000001_first");
    assert_eq!(migrations[1].number(), 3);
    assert_eq!(migrations[1].name(), "second");
    assert_eq!(migrations[1].directory(), "00000000000003_second");
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

#[test]
/// Test that the densification procedure works as expected.
pub fn test_densification() {
	// First, we duplicate the directory `standard_migrations` as
	// `standard_migrations_to_be_densified`, which we will densify.
	let source = Path::new("standard_migrations");
	let destination = Path::new("standard_migrations_to_be_densified");
	copy_dir_all(source, destination).unwrap();

	// We create the migration directory.
	let mut migrations = MigrationDirectory::try_from("standard_migrations_to_be_densified").unwrap();
	assert_eq!(migrations.len(), 2);
	assert!(!migrations.is_dense());

	// We densify the migrations.
	migrations = migrations.redensify().unwrap();
	assert_eq!(migrations.len(), 2);
	assert!(migrations.is_dense());

	// We reload the migrations from the directory.
	let migrations = MigrationDirectory::try_from("standard_migrations_to_be_densified").unwrap();
	assert_eq!(migrations.len(), 2);
	assert!(migrations.is_dense());

	// We remove the directory `standard_migrations_to_be_densified`.
	std::fs::remove_dir_all(destination).unwrap();
}
