//! Test submodule to verify whether the topological ordering and its detection
//! work as expected.

use std::path::Path;

use diesel_migrations_utils::prelude::*;

mod utils;
use utils::copy_dir_all;

#[test]
/// Test to check that the topological order of the standard migration
/// is determined as expected.
pub fn test_standard_migrations() {
    let standard = MigrationDirectory::try_from("standard_migrations").unwrap();
    assert!(standard.is_topologically_sorted().unwrap());
}

#[test]
/// Test to check that the out-of-order migrations are recognized as such.
pub fn test_out_of_order_migrations() {
    let out_of_order = MigrationDirectory::try_from("out_of_order_migrations").unwrap();
    assert!(!out_of_order.is_topologically_sorted().unwrap());
}

#[test]
/// Test to check that the out-of-order migrations are recognized as such.
pub fn test_out_of_order_migrations_topological_sorting() {
    // First, we duplicate the directory `out_of_order_migrations` as
    // `out_of_order_migrations_to_be_sorted`, which we will densify.
    let source = Path::new("out_of_order_migrations");
    let destination = Path::new("out_of_order_migrations_to_be_sorted");

    // If the destination directory already exists, we remove it.
    if destination.exists() {
        let _ = std::fs::remove_dir_all(destination);
    }

    copy_dir_all(source, destination).unwrap();

    // Then, we sort the migrations in `out_of_order_migrations_to_be_sorted`.
    let out_of_order =
        MigrationDirectory::try_from("out_of_order_migrations_to_be_sorted").unwrap();
    out_of_order.topologically_sort().unwrap();

    // Finally, we check that the migrations are now topologically sorted.
    let out_of_order =
        MigrationDirectory::try_from("out_of_order_migrations_to_be_sorted").unwrap();
    assert!(out_of_order.is_topologically_sorted().unwrap());

    // Clean up the copied directory.
    let _ = std::fs::remove_dir_all(destination);
}
